//! Heuristic bracketed-paste synthesis for hosts that do not emit the
//! bracketed-paste markers themselves.
//!
//! rmux normally relies on the HOST terminal to wrap pastes in
//! `\x1b[200~ … \x1b[201~`. Many SSH clients (e.g. Termius) do not, so a
//! multi-line paste arrives as one raw `read()` burst whose embedded `\r`/`\n`
//! bytes are each decoded by the server as an `Enter` keypress — submitting the
//! input mid-paste. To fix this at the source, the client inspects each raw
//! input burst: when the burst looks like a paste but is NOT already bracketed,
//! it synthesises the bracketed-paste wrapping so the server's existing
//! bracketed-paste handler forwards the body verbatim to the PTY (no per-newline
//! `Enter`).
//!
//! A genuine lone `Enter` keypress (`\r`, `\n`, or `\r\n`, optionally preceded by
//! a single typed character) must NOT be wrapped — those are small bursts whose
//! only newline is trailing, and the server's `key_table` still maps them to
//! `Enter` as before.

/// Bracketed-paste start marker.
pub(crate) const BRACKETED_PASTE_START: &[u8] = b"\x1b[200~";
/// Bracketed-paste end marker.
pub(crate) const BRACKETED_PASTE_END: &[u8] = b"\x1b[201~";

/// A raw burst at or above this size is treated as a paste even when it
/// contains no embedded newline (a fast paste of one long line, possibly with a
/// trailing submit newline, that would otherwise glitch or lag). Normal typing
/// — including held-down keys — does not deliver this many bytes in a single
/// `read()`.
const PASTE_SIZE_THRESHOLD: usize = 32;

/// Decide whether a raw input burst should be wrapped as a synthetic
/// bracketed paste, and return the bytes to forward.
///
/// Returns `None` when the burst should be forwarded unchanged (the common
/// keystroke path — zero allocation). Returns `Some(wrapped)` when the burst is
/// a heuristically-detected unbracketed paste and has been wrapped in
/// `\x1b[200~ … \x1b[201~`.
pub(crate) fn maybe_wrap_paste(burst: &[u8]) -> Option<Vec<u8>> {
    if !looks_like_unbracketed_paste(burst) {
        return None;
    }
    let mut wrapped =
        Vec::with_capacity(burst.len() + BRACKETED_PASTE_START.len() + BRACKETED_PASTE_END.len());
    wrapped.extend_from_slice(BRACKETED_PASTE_START);
    wrapped.extend_from_slice(burst);
    wrapped.extend_from_slice(BRACKETED_PASTE_END);
    Some(wrapped)
}

/// True when the burst looks like a paste that the host did NOT already bracket.
fn looks_like_unbracketed_paste(burst: &[u8]) -> bool {
    // Already bracketed by the host (or a marker is mid-stream) — never re-wrap;
    // the server handles real bracketed pastes already.
    if contains_subslice(burst, BRACKETED_PASTE_START)
        || contains_subslice(burst, BRACKETED_PASTE_END)
    {
        return false;
    }

    // A multi-line paste: a newline with further content after it in the same
    // burst. A lone `Enter` (`\r`, `\n`, `\r\n`) or a "typed char then Enter"
    // burst (`x\r`) has nothing after its final newline, so it is NOT wrapped.
    if has_content_after_newline(burst) {
        return true;
    }

    // A large single-burst with no embedded newline is a fast paste of one long
    // line (Termius delivers the whole clipboard in one `read()`); wrap it so a
    // trailing submit newline / control byte cannot glitch the input.
    burst.len() >= PASTE_SIZE_THRESHOLD
}

/// True when the burst contains a `\r` or `\n` that is followed by at least one
/// more byte within the same burst (i.e. the newline is not purely trailing).
fn has_content_after_newline(burst: &[u8]) -> bool {
    for (index, byte) in burst.iter().enumerate() {
        if (*byte == b'\r' || *byte == b'\n') && index + 1 < burst.len() {
            // Treat a `\r\n` pair as a single newline: a `\r` immediately
            // followed by `\n` is still "trailing" if the `\n` is the last byte.
            let next_is_lf_terminator =
                *byte == b'\r' && burst.get(index + 1) == Some(&b'\n') && index + 2 == burst.len();
            if !next_is_lf_terminator {
                return true;
            }
        }
    }
    false
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::{maybe_wrap_paste, BRACKETED_PASTE_END, BRACKETED_PASTE_START, PASTE_SIZE_THRESHOLD};

    fn wrapped(body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(BRACKETED_PASTE_START);
        out.extend_from_slice(body);
        out.extend_from_slice(BRACKETED_PASTE_END);
        out
    }

    #[test]
    fn lone_enter_is_not_wrapped() {
        assert_eq!(maybe_wrap_paste(b"\r"), None);
        assert_eq!(maybe_wrap_paste(b"\n"), None);
        assert_eq!(maybe_wrap_paste(b"\r\n"), None);
    }

    #[test]
    fn typed_char_then_enter_is_not_wrapped() {
        assert_eq!(maybe_wrap_paste(b"x\r"), None);
        assert_eq!(maybe_wrap_paste(b"x\r\n"), None);
    }

    #[test]
    fn single_keystroke_is_not_wrapped() {
        assert_eq!(maybe_wrap_paste(b"a"), None);
        assert_eq!(maybe_wrap_paste(b"\x1b[A"), None); // arrow up
    }

    #[test]
    fn multiline_paste_is_wrapped() {
        let body = b"line one\r\nline two";
        assert_eq!(maybe_wrap_paste(body), Some(wrapped(body)));
    }

    #[test]
    fn multiline_paste_with_trailing_newline_is_wrapped() {
        let body = b"line one\nline two\n";
        assert_eq!(maybe_wrap_paste(body), Some(wrapped(body)));
    }

    #[test]
    fn large_single_line_burst_is_wrapped() {
        let body = vec![b'x'; PASTE_SIZE_THRESHOLD];
        assert_eq!(maybe_wrap_paste(&body), Some(wrapped(&body)));
    }

    #[test]
    fn small_single_line_burst_is_not_wrapped() {
        let body = vec![b'x'; PASTE_SIZE_THRESHOLD - 1];
        assert_eq!(maybe_wrap_paste(&body), None);
    }

    #[test]
    fn already_bracketed_paste_is_not_rewrapped() {
        let body = wrapped(b"line one\r\nline two");
        assert_eq!(maybe_wrap_paste(&body), None);
    }

    #[test]
    fn burst_containing_only_end_marker_is_not_wrapped() {
        // A stray close marker must not trigger synthesis.
        assert_eq!(maybe_wrap_paste(BRACKETED_PASTE_END), None);
    }
}
