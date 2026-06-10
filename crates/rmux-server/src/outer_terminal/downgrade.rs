//! SGR downgrade pass applied at the client-emission boundary.
//!
//! The grid renderer emits truecolor (`38;2;r;g;b`) and colon-form SGR
//! (`4:2`..`4:5` underline styles, `5:3` overline) unconditionally — the tmux
//! capture format reused as the live render stream. A client whose terminal
//! lacks those capabilities (Termius reporting plain `xterm-256color`, for
//! example) renders them as garbage or not at all, so every rendered frame is
//! rewritten here against the attached client's resolved feature set before
//! it leaves the server. Clients with full support take a zero-copy fast path.

use std::borrow::Cow;

use super::colours::nearest_256_index;

/// The capability subset the SGR rewrite cares about.
#[derive(Debug, Clone, Copy)]
pub(super) struct SgrCaps {
    pub(super) rgb: bool,
    pub(super) usstyle: bool,
    pub(super) overline: bool,
}

impl SgrCaps {
    const fn full(self) -> bool {
        self.rgb && self.usstyle && self.overline
    }
}

/// Rewrites every SGR sequence in `frame` to forms `caps` can interpret.
/// Non-SGR sequences (cursor moves, OSC/DCS strings, graphics passthrough)
/// are copied verbatim.
pub(super) fn downgrade_sgr_frame<'frame>(frame: &'frame [u8], caps: SgrCaps) -> Cow<'frame, [u8]> {
    if caps.full() {
        return Cow::Borrowed(frame);
    }

    let mut output = Vec::with_capacity(frame.len());
    let mut index = 0;
    while index < frame.len() {
        if frame[index] != 0x1b {
            output.push(frame[index]);
            index += 1;
            continue;
        }
        match frame.get(index + 1) {
            Some(b'[') => {
                // CSI: scan to the final byte (0x40-0x7e); only SGR ('m') is
                // rewritten, every other CSI is copied untouched.
                let mut end = index + 2;
                while end < frame.len() && !(0x40..=0x7e).contains(&frame[end]) {
                    end += 1;
                }
                if end < frame.len() && frame[end] == b'm' {
                    rewrite_sgr(&frame[index + 2..end], caps, &mut output);
                    index = end + 1;
                } else {
                    let end = (end + 1).min(frame.len());
                    output.extend_from_slice(&frame[index..end]);
                    index = end;
                }
            }
            // OSC/DCS/APC/PM/SOS strings run to ST or BEL and may contain
            // arbitrary payload bytes — never rewrite inside them.
            Some(b']' | b'P' | b'_' | b'^' | b'X') => {
                let mut end = index + 2;
                while end < frame.len() {
                    if frame[end] == 0x07 {
                        end += 1;
                        break;
                    }
                    if frame[end] == 0x1b && frame.get(end + 1) == Some(&b'\\') {
                        end += 2;
                        break;
                    }
                    end += 1;
                }
                output.extend_from_slice(&frame[index..end]);
                index = end;
            }
            _ => {
                output.push(frame[index]);
                index += 1;
            }
        }
    }
    Cow::Owned(output)
}

/// Rewrites one SGR parameter string (the bytes between `ESC [` and `m`).
///
/// The renderer's SGR grammar is semicolon-separated parameters where
/// extended colours span five (`38;2;r;g;b`) or three (`38;5;n`) parameters
/// and attribute codes >= 10 use one colon-form token (`4:2`, `5:3`).
fn rewrite_sgr(params: &[u8], caps: SgrCaps, output: &mut Vec<u8>) {
    let valid = params
        .iter()
        .all(|byte| byte.is_ascii_digit() || *byte == b';' || *byte == b':');
    let Some(text) = std::str::from_utf8(params).ok().filter(|_| valid) else {
        output.extend_from_slice(b"\x1b[");
        output.extend_from_slice(params);
        output.push(b'm');
        return;
    };

    let tokens = text.split(';').collect::<Vec<_>>();
    let mut kept = Vec::with_capacity(tokens.len());
    let mut index = 0;
    while index < tokens.len() {
        let token = tokens[index];
        let is_extended_colour = matches!(token, "38" | "48" | "58");
        if is_extended_colour && index + 4 < tokens.len() && tokens[index + 1] == "2" {
            let channels = (
                tokens[index + 2].parse::<u8>(),
                tokens[index + 3].parse::<u8>(),
                tokens[index + 4].parse::<u8>(),
            );
            if token == "58" && !caps.usstyle {
                // Underline colour needs Smulx-style support; drop it.
            } else if let (Ok(r), Ok(g), Ok(b)) = channels {
                if caps.rgb {
                    kept.extend(tokens[index..index + 5].iter().map(|t| (*t).to_owned()));
                } else {
                    kept.push(format!("{token};5;{}", nearest_256_index(r, g, b)));
                }
            } else {
                // Out-of-range channel: keep verbatim rather than guess.
                kept.extend(tokens[index..index + 5].iter().map(|t| (*t).to_owned()));
            }
            index += 5;
            continue;
        }
        if is_extended_colour && index + 2 < tokens.len() && tokens[index + 1] == "5" {
            if token != "58" || caps.usstyle {
                kept.extend(tokens[index..index + 3].iter().map(|t| (*t).to_owned()));
            }
            index += 3;
            continue;
        }
        match token {
            // Underline-colour reset travels with usstyle support.
            "59" if !caps.usstyle => {}
            _ if token.contains(':') => match token.split_once(':') {
                // 4:0 (no underline) and 4:1..4:5 (underline styles): a
                // client without usstyle still understands plain on/off.
                Some(("4", style)) => {
                    if caps.usstyle {
                        kept.push(token.to_owned());
                    } else {
                        kept.push(if style == "0" { "24" } else { "4" }.to_owned());
                    }
                }
                // Overline reaches the wire as colon-form "5:3" (the
                // renderer's encoding of code 53); real terminals expect
                // plain CSI 53 m, or nothing when unsupported.
                Some(("5", "3")) => {
                    if caps.overline {
                        kept.push("53".to_owned());
                    }
                }
                _ => kept.push(token.to_owned()),
            },
            _ => kept.push(token.to_owned()),
        }
        index += 1;
    }

    // Every parameter was dropped: emitting bare `ESC [ m` would mean a full
    // reset the renderer never asked for, so emit nothing instead.
    if kept.is_empty() && !tokens.is_empty() && tokens != [""] {
        return;
    }
    output.extend_from_slice(b"\x1b[");
    output.extend_from_slice(kept.join(";").as_bytes());
    output.push(b'm');
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{downgrade_sgr_frame, SgrCaps};

    const FULL: SgrCaps = SgrCaps {
        rgb: true,
        usstyle: true,
        overline: true,
    };
    const BARE_256: SgrCaps = SgrCaps {
        rgb: false,
        usstyle: false,
        overline: false,
    };

    #[test]
    fn full_capability_clients_take_the_borrowed_fast_path() {
        let frame = b"\x1b[38;2;10;20;30mhi";
        assert!(matches!(
            downgrade_sgr_frame(frame, FULL),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn truecolor_downgrades_to_nearest_256_for_non_rgb_clients() {
        // 0x5f/0x87/0xaf are exact cube levels -> 16 + 36*1 + 6*2 + 3 = 67.
        let frame = b"\x1b[38;2;95;135;175mX\x1b[48;2;0;0;0mY";
        let rewritten = downgrade_sgr_frame(frame, BARE_256);
        assert_eq!(rewritten.as_ref(), b"\x1b[38;5;67mX\x1b[48;5;16mY");
    }

    #[test]
    fn colon_underline_styles_become_plain_underline_without_usstyle() {
        let frame = b"\x1b[1;4:3;38;5;10mZ";
        let rewritten = downgrade_sgr_frame(frame, BARE_256);
        assert_eq!(rewritten.as_ref(), b"\x1b[1;4;38;5;10mZ");
    }

    #[test]
    fn underline_colour_drops_without_usstyle_and_overline_drops_entirely() {
        let frame = b"\x1b[58;2;1;2;3;5:3mW";
        let rewritten = downgrade_sgr_frame(frame, BARE_256);
        assert_eq!(rewritten.as_ref(), b"W");
    }

    #[test]
    fn overline_rewrites_to_plain_53_when_supported() {
        let caps = SgrCaps {
            rgb: true,
            usstyle: true,
            overline: false,
        };
        // Force the rewrite path (overline false) on a frame without overline.
        let frame = b"\x1b[5:3m\x1b[31mA";
        assert_eq!(downgrade_sgr_frame(frame, caps).as_ref(), b"\x1b[31mA");

        let caps = SgrCaps {
            rgb: false,
            usstyle: false,
            overline: true,
        };
        let frame = b"\x1b[5:3mA";
        assert_eq!(downgrade_sgr_frame(frame, caps).as_ref(), b"\x1b[53mA");
    }

    #[test]
    fn non_sgr_sequences_and_osc_strings_pass_through_untouched() {
        // Cursor save/position CSIs and the OSC title payload (including its
        // embedded fake-SGR bytes) must come through byte-identical.
        let frame = b"\x1b[s\x1b[2;1H\x1b]0;ti\x1b[38;2;1;1;1mtle\x07\x1b[0mB\x1b[u";
        let rewritten = downgrade_sgr_frame(frame, BARE_256);
        assert_eq!(rewritten.as_ref(), frame.as_slice());
    }

    #[test]
    fn reset_and_basic_colours_are_preserved() {
        let frame = b"\x1b[0m\x1b[31;42;7mC";
        let rewritten = downgrade_sgr_frame(frame, BARE_256);
        assert_eq!(rewritten.as_ref(), frame.as_slice());
    }
}
