//! Heuristic bracketed-paste synthesis for hosts that do not emit the
//! bracketed-paste markers themselves — and stateful tracking so a paste
//! larger than one `read()` burst stays a single bracketed block.
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
//!
//! ## Why the filter is stateful
//!
//! A large paste (≳4 KB) never arrives in one `read()`: the kernel tty layer
//! delivers it as several bursts. Per-burst classification corrupted those
//! pastes two ways:
//!
//! * **Host-bracketed paste**: burst 1 carries `\x1b[200~` + the head (left
//!   alone — correct), but a MIDDLE burst is pure body with no marker, so the
//!   per-burst heuristic re-wrapped it in its own `200~…201~`. The synthetic
//!   `201~` closed the host paste early on the server; the rest of the paste
//!   was forwarded raw and every embedded newline submitted a command.
//! * **Unbracketed paste**: each burst became its own bracketed block, so one
//!   paste reached the pane app as several separate pastes.
//!
//! [`PasteFilter`] therefore tracks state across bursts: while a host paste is
//! open it forwards everything verbatim and never synthesises; when it opens a
//! synthetic paste it keeps it open across subsequent bursts (they are body,
//! forwarded verbatim) and only emits the closing `\x1b[201~` once the input
//! has been quiet for [`SYNTH_PASTE_QUIET_MS`] — one paste in, one bracketed
//! block out, regardless of size.

/// Bracketed-paste start marker.
pub(crate) const BRACKETED_PASTE_START: &[u8] = b"\x1b[200~";
/// Bracketed-paste end marker.
pub(crate) const BRACKETED_PASTE_END: &[u8] = b"\x1b[201~";

/// Both markers are the same length.
const MARKER_LEN: usize = 6;

/// Quiet window (no input readable) after the last burst of a synthesised
/// paste before the closing marker is emitted. Continuation bursts of one
/// physical paste arrive within microseconds-to-low-milliseconds of each
/// other; 25 ms catches them with ample headroom while staying imperceptible
/// before the pane app sees the paste complete.
pub(crate) const SYNTH_PASTE_QUIET_MS: u64 = 25;

/// A raw burst at or above this size is treated as a paste even when it
/// contains no embedded newline (a fast paste of one long line, possibly with a
/// trailing submit newline, that would otherwise glitch or lag). Normal typing
/// — including held-down keys — does not deliver this many bytes in a single
/// `read()`.
const PASTE_SIZE_THRESHOLD: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PasteFilterState {
    /// No paste in progress — per-burst classification applies.
    Idle,
    /// A host-bracketed paste is open (`\x1b[200~` seen, `\x1b[201~` not yet):
    /// forward everything verbatim, never synthesise.
    HostOpen,
    /// A synthesised paste is open: bursts are body, forwarded verbatim; the
    /// closing marker is emitted on quiet.
    SynthOpen,
}

/// Stateful paste classifier for the attach input loop.
///
/// Feed every raw input burst through [`PasteFilter::on_burst`] and forward
/// its return value. When the input stays quiet for [`SYNTH_PASTE_QUIET_MS`]
/// (poll timeout), call [`PasteFilter::on_quiet`] and forward the closing
/// marker it returns, if any. [`PasteFilter::synth_open`] tells the loop when
/// to poll with the short quiet timeout.
#[derive(Debug)]
pub(crate) struct PasteFilter {
    state: PasteFilterState,
    /// Last `MARKER_LEN - 1` bytes of the previous burst, kept so a
    /// bracketed-paste marker split across two `read()` bursts is still
    /// recognised. Detection only — these bytes are never re-forwarded.
    carry: Vec<u8>,
}

impl PasteFilter {
    pub(crate) fn new() -> Self {
        Self {
            state: PasteFilterState::Idle,
            carry: Vec::new(),
        }
    }

    /// True while a synthesised paste is open — the caller should poll input
    /// with the [`SYNTH_PASTE_QUIET_MS`] timeout and call
    /// [`PasteFilter::on_quiet`] when it expires.
    pub(crate) fn synth_open(&self) -> bool {
        self.state == PasteFilterState::SynthOpen
    }

    /// Drop all state (e.g. when the attach stream locks — the server clears
    /// its pending input too, so an open paste is a lost cause).
    pub(crate) fn reset(&mut self) {
        self.state = PasteFilterState::Idle;
        self.carry.clear();
    }

    /// Process one raw input burst and return the bytes to forward.
    ///
    /// The common keystroke path returns the burst unchanged. When the burst
    /// opens a synthetic paste, the start marker is prepended; the closing
    /// marker is emitted later by [`PasteFilter::on_quiet`].
    pub(crate) fn on_burst(&mut self, burst: &[u8]) -> Vec<u8> {
        // Scan buffer (carry + burst) is for marker DETECTION only; only the
        // burst bytes themselves are ever forwarded. A complete marker cannot
        // fit inside the carry alone (MARKER_LEN - 1 bytes), so every marker
        // found here ends in this burst and is counted exactly once.
        let mut scan = Vec::with_capacity(self.carry.len() + burst.len());
        scan.extend_from_slice(&self.carry);
        scan.extend_from_slice(burst);

        let was_host_open = self.state == PasteFilterState::HostOpen;
        let host_open_after = scan_marker_state(was_host_open, &scan);
        let saw_marker = contains_subslice(&scan, BRACKETED_PASTE_START)
            || contains_subslice(&scan, BRACKETED_PASTE_END);

        let out = match self.state {
            PasteFilterState::SynthOpen => {
                if burst.first() == Some(&0x1b) || saw_marker {
                    // Terminal event traffic (mouse, keys) or a host marker
                    // arriving while a synthetic paste is open: close the
                    // synthetic block first so those bytes are not swallowed
                    // as paste body.
                    self.state = if host_open_after {
                        PasteFilterState::HostOpen
                    } else {
                        PasteFilterState::Idle
                    };
                    let mut out = Vec::with_capacity(MARKER_LEN + burst.len());
                    out.extend_from_slice(BRACKETED_PASTE_END);
                    out.extend_from_slice(burst);
                    out
                } else {
                    // Continuation burst of the same physical paste — body,
                    // forwarded verbatim inside the open synthetic block.
                    burst.to_vec()
                }
            }
            PasteFilterState::HostOpen => {
                // Inside a host-bracketed paste: everything is body (or the
                // closing marker) — forward verbatim, never synthesise.
                self.state = if host_open_after {
                    PasteFilterState::HostOpen
                } else {
                    PasteFilterState::Idle
                };
                burst.to_vec()
            }
            PasteFilterState::Idle => {
                if host_open_after {
                    // The host opened a bracketed paste (marker possibly split
                    // across bursts — the carry catches that): pass through.
                    self.state = PasteFilterState::HostOpen;
                    burst.to_vec()
                } else if saw_marker {
                    // A complete host-bracketed paste (or stray marker) within
                    // this burst: the server handles real markers already.
                    burst.to_vec()
                } else if looks_like_unbracketed_paste(burst) {
                    // Open a synthetic paste: start marker + body now, closing
                    // marker on quiet — continuation bursts of a large paste
                    // join this same block instead of becoming separate pastes.
                    self.state = PasteFilterState::SynthOpen;
                    let mut out = Vec::with_capacity(MARKER_LEN + burst.len());
                    out.extend_from_slice(BRACKETED_PASTE_START);
                    out.extend_from_slice(burst);
                    out
                } else {
                    burst.to_vec()
                }
            }
        };

        // Keep the last MARKER_LEN - 1 HOST bytes for split-marker detection.
        let keep = scan.len().min(MARKER_LEN - 1);
        self.carry = scan[scan.len() - keep..].to_vec();

        out
    }

    /// The input has been quiet past [`SYNTH_PASTE_QUIET_MS`]: close an open
    /// synthetic paste. Returns the closing marker to forward, if any.
    pub(crate) fn on_quiet(&mut self) -> Option<Vec<u8>> {
        if self.state != PasteFilterState::SynthOpen {
            return None;
        }
        self.state = PasteFilterState::Idle;
        self.carry.clear();
        Some(BRACKETED_PASTE_END.to_vec())
    }
}

/// Walk `hay` for bracketed-paste markers in order; the last marker seen
/// decides whether a host paste is open afterwards.
fn scan_marker_state(initially_open: bool, hay: &[u8]) -> bool {
    let mut open = initially_open;
    let mut index = 0;
    while index + MARKER_LEN <= hay.len() {
        if &hay[index..index + MARKER_LEN] == BRACKETED_PASTE_START {
            open = true;
            index += MARKER_LEN;
        } else if &hay[index..index + MARKER_LEN] == BRACKETED_PASTE_END {
            open = false;
            index += MARKER_LEN;
        } else {
            index += 1;
        }
    }
    open
}

/// True when the burst looks like a paste that the host did NOT already bracket.
fn looks_like_unbracketed_paste(burst: &[u8]) -> bool {
    // A burst that BEGINS with ESC is terminal event traffic, not a paste:
    // SGR mouse reports (`\x1b[<64;x;yM` — a wheel flick batches several per
    // read(), easily exceeding PASTE_SIZE_THRESHOLD with no newline), key
    // autorepeat (`\x1b[A\x1b[A…`), focus events, etc. Clipboard text starting
    // with a raw ESC byte does not occur in practice. Wrapping these as a
    // paste sends them to the PTY as literal text — killing mouse scroll.
    if burst.first() == Some(&0x1b) {
        return false;
    }

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
    use super::{
        PasteFilter, BRACKETED_PASTE_END, BRACKETED_PASTE_START, PASTE_SIZE_THRESHOLD,
    };

    fn start_plus(body: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(BRACKETED_PASTE_START);
        out.extend_from_slice(body);
        out
    }

    fn wrapped(body: &[u8]) -> Vec<u8> {
        let mut out = start_plus(body);
        out.extend_from_slice(BRACKETED_PASTE_END);
        out
    }

    /// Single burst through a fresh filter, closed by quiet — the stateful
    /// equivalent of the old `maybe_wrap_paste(burst) == Some(wrapped)`.
    fn synthesised(burst: &[u8]) -> Option<Vec<u8>> {
        let mut filter = PasteFilter::new();
        let out = filter.on_burst(burst);
        match filter.on_quiet() {
            Some(end) => {
                let mut full = out;
                full.extend_from_slice(&end);
                Some(full)
            }
            None => {
                assert_eq!(out, burst.to_vec(), "non-paste burst must pass verbatim");
                None
            }
        }
    }

    #[test]
    fn lone_enter_is_not_wrapped() {
        assert_eq!(synthesised(b"\r"), None);
        assert_eq!(synthesised(b"\n"), None);
        assert_eq!(synthesised(b"\r\n"), None);
    }

    #[test]
    fn typed_char_then_enter_is_not_wrapped() {
        assert_eq!(synthesised(b"x\r"), None);
        assert_eq!(synthesised(b"x\r\n"), None);
    }

    #[test]
    fn single_keystroke_is_not_wrapped() {
        assert_eq!(synthesised(b"a"), None);
        assert_eq!(synthesised(b"\x1b[A"), None); // arrow up
    }

    #[test]
    fn multiline_paste_is_wrapped() {
        let body = b"line one\r\nline two";
        assert_eq!(synthesised(body), Some(wrapped(body)));
    }

    #[test]
    fn multiline_paste_with_trailing_newline_is_wrapped() {
        let body = b"line one\nline two\n";
        assert_eq!(synthesised(body), Some(wrapped(body)));
    }

    #[test]
    fn large_single_line_burst_is_wrapped() {
        let body = vec![b'x'; PASTE_SIZE_THRESHOLD];
        assert_eq!(synthesised(&body), Some(wrapped(&body)));
    }

    #[test]
    fn small_single_line_burst_is_not_wrapped() {
        let body = vec![b'x'; PASTE_SIZE_THRESHOLD - 1];
        assert_eq!(synthesised(&body), None);
    }

    #[test]
    fn mouse_scroll_burst_is_not_wrapped() {
        // A wheel flick delivers several SGR mouse reports in one read() —
        // easily >= PASTE_SIZE_THRESHOLD bytes with no newline. Wrapping them
        // as a paste sends them to the PTY as literal text and kills scroll.
        let burst = b"\x1b[<64;42;10M\x1b[<64;42;10M\x1b[<65;42;10M";
        assert!(burst.len() >= PASTE_SIZE_THRESHOLD);
        assert_eq!(synthesised(burst), None);
    }

    #[test]
    fn arrow_key_autorepeat_burst_is_not_wrapped() {
        // Held-down arrow key: terminal autorepeat can batch many CSI
        // sequences into a single read() burst.
        let burst = b"\x1b[A".repeat(12);
        assert!(burst.len() >= PASTE_SIZE_THRESHOLD);
        assert_eq!(synthesised(&burst), None);
    }

    #[test]
    fn key_sequence_with_embedded_enter_is_not_wrapped() {
        // Up, Enter, Up batched into one burst over a slow link is keyboard
        // traffic, not a paste — the embedded newline must not trigger wrap.
        assert_eq!(synthesised(b"\x1b[A\r\x1b[A"), None);
    }

    #[test]
    fn already_bracketed_paste_is_not_rewrapped() {
        let body = wrapped(b"line one\r\nline two");
        assert_eq!(synthesised(&body), None);
    }

    #[test]
    fn burst_containing_only_end_marker_is_not_wrapped() {
        // A stray close marker must not trigger synthesis.
        assert_eq!(synthesised(BRACKETED_PASTE_END), None);
    }

    // ------------------------------------------------------------------
    // Stateful behaviour: pastes larger than one read() burst.
    // ------------------------------------------------------------------

    /// THE 10k-paste regression: a host-bracketed paste split across three
    /// bursts. The middle burst is pure multi-line body with no marker; the
    /// old per-burst heuristic re-wrapped it, and the synthetic `201~` closed
    /// the host paste early on the server.
    #[test]
    fn middle_burst_of_open_host_paste_is_never_rewrapped() {
        let mut filter = PasteFilter::new();

        let burst1 = start_plus(b"head of a very long paste\nstill head\n");
        let burst2 = b"middle of the paste\nwith newlines\nand more body\n".to_vec();
        let mut burst3 = b"tail of the paste\n".to_vec();
        burst3.extend_from_slice(BRACKETED_PASTE_END);

        assert_eq!(filter.on_burst(&burst1), burst1);
        assert_eq!(filter.on_burst(&burst2), burst2); // verbatim — NOT wrapped
        assert_eq!(filter.on_burst(&burst3), burst3);
        // Paste closed: classification is back to normal.
        assert_eq!(filter.on_quiet(), None);
        let body = b"after\nthe\npaste";
        assert_eq!(filter.on_burst(body), start_plus(body));
        assert!(filter.synth_open());
    }

    /// Host paste whose START marker is split across two bursts: the carry
    /// must still recognise the paste as host-bracketed.
    #[test]
    fn host_start_marker_split_across_bursts_suppresses_synthesis() {
        let mut filter = PasteFilter::new();

        let burst1 = b"\x1b[20"; // first half of \x1b[200~
        let mut burst2 = b"0~".to_vec(); // second half
        burst2.extend_from_slice(b"long body\nwith newlines\nthat must not be wrapped\n");

        assert_eq!(filter.on_burst(burst1), burst1.to_vec());
        assert_eq!(filter.on_burst(&burst2), burst2); // host paste open — verbatim
        assert!(!filter.synth_open());

        let mut burst3 = b"tail".to_vec();
        burst3.extend_from_slice(BRACKETED_PASTE_END);
        assert_eq!(filter.on_burst(&burst3), burst3);
        assert_eq!(filter.on_quiet(), None);
    }

    /// Host paste whose END marker is split across two bursts: the filter must
    /// notice the close and resume normal classification afterwards.
    #[test]
    fn host_end_marker_split_across_bursts_closes_the_paste() {
        let mut filter = PasteFilter::new();

        let burst1 = start_plus(b"body\nbody\n");
        let burst2 = b"more body\x1b[20"; // END split: \x1b[20 | 1~
        let burst3 = b"1~";

        assert_eq!(filter.on_burst(&burst1), burst1);
        assert_eq!(filter.on_burst(burst2), burst2.to_vec());
        assert_eq!(filter.on_burst(burst3), burst3.to_vec());
        assert!(!filter.synth_open());

        // Closed: a new multi-line burst is a fresh unbracketed paste again.
        let body = b"new\npaste";
        let out = filter.on_burst(body);
        assert_eq!(out, start_plus(body));
        assert!(filter.synth_open());
    }

    /// An unbracketed paste split across three bursts must reach the server as
    /// ONE bracketed block: START + body bursts verbatim + END on quiet.
    #[test]
    fn split_unbracketed_paste_becomes_a_single_block() {
        let mut filter = PasteFilter::new();

        let burst1 = b"first chunk of a 10k paste\nline\nline\n";
        let burst2 = b"second chunk, still the same paste\nline\n";
        let burst3 = b"third chunk, tail\n";

        assert_eq!(filter.on_burst(burst1), start_plus(burst1));
        assert!(filter.synth_open());
        assert_eq!(filter.on_burst(burst2), burst2.to_vec()); // body, no extra markers
        assert_eq!(filter.on_burst(burst3), burst3.to_vec());
        assert_eq!(filter.on_quiet(), Some(BRACKETED_PASTE_END.to_vec()));
        assert!(!filter.synth_open());
        assert_eq!(filter.on_quiet(), None); // idempotent
    }

    /// Terminal event traffic (ESC-initiated, e.g. a wheel flick) arriving
    /// while a synthetic paste is still open must close the paste first, not
    /// be swallowed as paste body.
    #[test]
    fn mouse_burst_during_open_synth_paste_closes_it_first() {
        let mut filter = PasteFilter::new();

        let paste = b"pasted\nmulti\nline";
        assert_eq!(filter.on_burst(paste), start_plus(paste));
        assert!(filter.synth_open());

        let mouse = b"\x1b[<64;42;10M\x1b[<64;42;10M";
        let mut expected = BRACKETED_PASTE_END.to_vec();
        expected.extend_from_slice(mouse);
        assert_eq!(filter.on_burst(mouse), expected);
        assert!(!filter.synth_open());
        assert_eq!(filter.on_quiet(), None);
    }

    /// `reset()` (attach lock) drops an open paste without emitting markers.
    #[test]
    fn reset_drops_open_synth_paste() {
        let mut filter = PasteFilter::new();
        let paste = b"pasted\nmulti\nline";
        assert_eq!(filter.on_burst(paste), start_plus(paste));
        filter.reset();
        assert!(!filter.synth_open());
        assert_eq!(filter.on_quiet(), None);
    }
}
