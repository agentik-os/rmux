//! Canonical identity newtypes shared across the RMUX workspace.
//!
//! `rmux-proto` is the single public home for the identity vocabulary
//! (`SessionName`, `SessionId`, `WindowId`, `PaneId`). Other crates,
//! including `rmux-core`, `rmux-server`, and `rmux-sdk`, must re-export
//! these types rather than declaring their own. Allocation, lookup, and
//! resolution remain in `rmux-core::session`; the types defined here
//! describe identity values, not the policy that issues them.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use crate::RmuxError;

/// Maximum number of characters retained in a sanitized session name.
///
/// Long names bloat every status line, format expansion, and target-error
/// message; 64 is comfortably longer than any human-chosen name while
/// keeping a hostile/garbage name from poisoning the session panel.
pub const SESSION_NAME_MAX_CHARS: usize = 64;

/// A validated RMUX session name.
///
/// Empty strings are rejected. The stored name is guaranteed to be
/// **always-targetable**: it contains only printable, non-control
/// characters and never a backslash, so it round-trips through
/// `kill-session -t`, `list-panes -t`, and `display-message -t` without
/// re-escaping.
///
/// Sanitization rules (see [`sanitize_session_name`]):
/// - `:` and `.` are rewritten to `_` — they are target-syntax separators
///   (`session:window.pane`) and must never appear inside a name.
/// - Backslashes, ASCII/Unicode control characters, and whitespace control
///   (newline / tab / CR) are rewritten to `-`. Printable Unicode (e.g.
///   `é`) is preserved verbatim.
/// - Leading/trailing `-` introduced by sanitization are trimmed, and the
///   result is capped at [`SESSION_NAME_MAX_CHARS`] characters.
///
/// This makes sanitization **idempotent**: a name that is already clean
/// re-sanitizes to itself, which is what keeps `-t <name>` target
/// resolution from corrupting an already-stored name on lookup.
///
/// Historical note: earlier versions emitted tmux `vis`-style escape
/// sequences (`\303\251` for a UTF-8 `é`, `\001` for control bytes) into
/// the *stored* name. Those literal backslashes made the name
/// un-targetable — every target layer re-escaped `\` to `\\` and the
/// lookup never matched. `vis`-style escaping is a *display* concern
/// (`rmux-core::vis`, formats output) and is deliberately kept out of the
/// canonical identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct SessionName(String);

impl SessionName {
    /// Validates and stores a session name, sanitizing it to an
    /// always-targetable form. Empty input — or input that sanitizes to an
    /// empty string — is rejected; every other input is mapped to a safe,
    /// non-empty name.
    pub fn new(value: impl Into<String>) -> Result<Self, RmuxError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RmuxError::EmptySessionName);
        }

        let sanitized = sanitize_session_name(&value);
        if sanitized.is_empty() {
            return Err(RmuxError::EmptySessionName);
        }

        Ok(Self(sanitized))
    }

    /// Returns the sanitized validated session name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the sanitized string.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Rewrites an arbitrary user-supplied string into an always-targetable
/// session name. See [`SessionName`] for the rule set.
///
/// Input is a valid `&str` (callers that start from raw bytes decode with
/// `String::from_utf8_lossy` first, mapping malformed bytes to `U+FFFD`,
/// which is printable and therefore preserved). The function is idempotent:
/// `sanitize(sanitize(x)) == sanitize(x)`.
fn sanitize_session_name(input: &str) -> String {
    let mut sanitized = String::with_capacity(input.len());

    for ch in input.chars() {
        let mapped = match ch {
            // Target-syntax separators are never allowed inside a name.
            ':' | '.' => '_',
            // Backslashes corrupt every target layer (re-escape to `\\`),
            // control characters break single-line display and enumeration.
            '\\' => '-',
            c if c.is_control() => '-',
            // Everything else printable (incl. Unicode like `é`) is kept.
            c => c,
        };
        sanitized.push(mapped);
    }

    // Trim cosmetic leading/trailing `-` introduced by sanitization, then
    // cap length by characters (not bytes) so we never split a UTF-8 scalar.
    let trimmed = sanitized.trim_matches('-');
    trimmed.chars().take(SESSION_NAME_MAX_CHARS).collect()
}

impl AsRef<str> for SessionName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SessionName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SessionName {
    type Err = RmuxError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for SessionName {
    type Error = RmuxError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for SessionName {
    type Error = RmuxError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'de> Deserialize<'de> for SessionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Stable per-server session identity (`$N`).
///
/// `SessionId` is the numeric identity rendered as `$N` by tmux-compatible
/// formats. Allocation lives in `rmux-core::session::SessionStore`; the
/// type defined here is the storable, transferable identity value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(u32);

impl SessionId {
    /// Wraps a raw stable session identity.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw stable session identity.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "${}", self.0)
    }
}

impl From<SessionId> for u32 {
    fn from(value: SessionId) -> Self {
        value.0
    }
}

impl From<u32> for SessionId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Stable per-server window identity (`@N`).
///
/// `WindowId` is the numeric identity rendered as `@N` by tmux-compatible
/// formats. Allocation lives in `rmux-core::session`; the type defined
/// here is the storable, transferable identity value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowId(u32);

impl WindowId {
    /// Wraps a raw stable window identity.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw stable window identity.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for WindowId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "@{}", self.0)
    }
}

impl From<WindowId> for u32 {
    fn from(value: WindowId) -> Self {
        value.0
    }
}

impl From<u32> for WindowId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Stable per-server pane identity (`%N`).
///
/// `PaneId` is the numeric identity rendered as `%N` by tmux-compatible
/// formats. Allocation lives in `rmux-core::session::SessionStore`; the
/// type defined here is the storable, transferable identity value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PaneId(u32);

impl PaneId {
    /// Wraps a raw stable pane identity.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the raw stable pane identity.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Display for PaneId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "%{}", self.0)
    }
}

impl From<PaneId> for u32 {
    fn from(value: PaneId) -> Self {
        value.0
    }
}

impl From<u32> for PaneId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{PaneId, SessionId, SessionName, WindowId, SESSION_NAME_MAX_CHARS};
    use crate::RmuxError;

    #[test]
    fn session_name_rejects_empty_values() {
        assert_eq!(SessionName::new(""), Err(RmuxError::EmptySessionName));
    }

    #[test]
    fn session_name_rewrites_colon_and_dot() {
        assert_eq!(
            SessionName::new("alpha:beta.gamma")
                .expect("rewritten")
                .as_str(),
            "alpha_beta_gamma"
        );
    }

    #[test]
    fn session_name_round_trips_through_serde() {
        let payload = bincode::serialize("alpha.beta").expect("string encodes");
        assert_eq!(
            bincode::deserialize::<SessionName>(&payload).expect("rewritten on the wire"),
            SessionName::new("alpha_beta").expect("valid")
        );
    }

    #[test]
    fn session_name_serde_rejects_empty_payloads_truthfully() {
        let payload = bincode::serialize("").expect("empty string encodes");
        assert!(
            bincode::deserialize::<SessionName>(&payload).is_err(),
            "empty session names must fail deserialization rather than silently \
             producing an empty inner value"
        );
    }

    #[test]
    fn session_name_serialize_round_trips_after_rewriting() {
        let original = SessionName::new("alpha.beta").expect("rewrites dots");
        let bytes = bincode::serialize(&original).expect("session name encodes");
        let restored: SessionName =
            bincode::deserialize(&bytes).expect("session name decodes idempotently");
        assert_eq!(restored, original);
        assert_eq!(restored.as_str(), "alpha_beta");
    }

    #[test]
    fn session_name_from_str_and_try_from_match_constructor() {
        let from_str: SessionName = "alpha:beta".parse().expect("FromStr rewrites");
        let try_from_ref: SessionName =
            SessionName::try_from("alpha:beta").expect("TryFrom<&str> rewrites");
        let try_from_owned: SessionName =
            SessionName::try_from(String::from("alpha:beta")).expect("TryFrom<String> rewrites");
        assert_eq!(from_str, try_from_ref);
        assert_eq!(from_str, try_from_owned);
        assert_eq!(from_str.as_str(), "alpha_beta");
    }

    #[test]
    fn session_name_into_inner_returns_sanitized_string() {
        let owned = SessionName::new("alpha:beta")
            .expect("rewrites colons")
            .into_inner();
        assert_eq!(owned, "alpha_beta");
    }

    #[test]
    fn session_name_never_stores_backslashes_or_controls() {
        // The exact shape of the bug: a UTF-8 name with an accented char,
        // spaces, parens, and a trailing space. Under the old escaper this
        // became the un-targetable literal `...class\303\251s...`.
        let name = SessionName::new("Causioais trous (classés par impact) _")
            .expect("sanitizes to a clean name");
        let stored = name.as_str();
        assert!(
            !stored.contains('\\'),
            "stored name must never contain a backslash: {stored:?}"
        );
        assert!(
            !stored.chars().any(char::is_control),
            "stored name must never contain a control char: {stored:?}"
        );
        // The accented char is preserved verbatim (no octal escaping).
        assert_eq!(stored, "Causioais trous (classés par impact) _");
    }

    #[test]
    fn session_name_sanitization_is_idempotent() {
        // The double-escape / re-escape bug: re-sanitizing a stored name
        // must yield the same name, so `-t <stored>` lookups match.
        for raw in [
            "Causioais trous (classés par impact) _",
            "a\u{1}\u{7f}é",
            "weird\\name\twith\nbreaks",
            "alpha:beta.gamma",
        ] {
            let first = SessionName::new(raw).expect("sanitizes");
            let second = SessionName::new(first.as_str()).expect("re-sanitizes");
            assert_eq!(
                first, second,
                "sanitization must be idempotent for {raw:?}"
            );
        }
    }

    #[test]
    fn session_name_rewrites_backslash_and_control_to_dash() {
        let name = SessionName::new(String::from_utf8_lossy(b"a\x01\x7f\\b").into_owned())
            .expect("sanitizes");
        // a, control(0x01)->'-', del(0x7f)->'-', backslash->'-', b
        assert_eq!(name.as_str(), "a---b");
    }

    #[test]
    fn session_name_rejects_input_that_sanitizes_to_empty() {
        // A name made entirely of control chars / separators trims to empty
        // and must be rejected rather than producing a blank target.
        assert_eq!(
            SessionName::new("\u{1}\u{2}\u{3}"),
            Err(RmuxError::EmptySessionName)
        );
    }

    #[test]
    fn session_name_caps_length_at_max_chars() {
        let long = "x".repeat(SESSION_NAME_MAX_CHARS + 50);
        let name = SessionName::new(long).expect("sanitizes");
        assert_eq!(name.as_str().chars().count(), SESSION_NAME_MAX_CHARS);
    }

    #[test]
    fn session_id_displays_with_dollar_prefix() {
        assert_eq!(SessionId::new(7).to_string(), "$7");
        assert_eq!(SessionId::new(7).as_u32(), 7);
    }

    #[test]
    fn window_id_displays_with_at_prefix() {
        assert_eq!(WindowId::new(9).to_string(), "@9");
        assert_eq!(WindowId::new(9).as_u32(), 9);
    }

    #[test]
    fn window_id_zero_and_max_render_as_at_prefixed_decimal() {
        assert_eq!(WindowId::new(0).to_string(), "@0");
        assert_eq!(
            WindowId::new(u32::MAX).to_string(),
            format!("@{}", u32::MAX)
        );
    }

    #[test]
    fn pane_id_displays_with_percent_prefix() {
        assert_eq!(PaneId::new(3).to_string(), "%3");
        assert_eq!(PaneId::new(3).as_u32(), 3);
    }

    #[test]
    fn pane_id_zero_and_max_render_as_percent_prefixed_decimal() {
        assert_eq!(PaneId::new(0).to_string(), "%0");
        assert_eq!(PaneId::new(u32::MAX).to_string(), format!("%{}", u32::MAX));
    }

    #[test]
    fn session_id_zero_and_max_render_as_dollar_prefixed_decimal() {
        assert_eq!(SessionId::new(0).to_string(), "$0");
        assert_eq!(
            SessionId::new(u32::MAX).to_string(),
            format!("${}", u32::MAX)
        );
    }

    #[test]
    fn identity_newtypes_round_trip_through_u32_conversions() {
        for value in [0_u32, 1, 17, u32::MAX] {
            assert_eq!(u32::from(SessionId::from(value)), value);
            assert_eq!(u32::from(WindowId::from(value)), value);
            assert_eq!(u32::from(PaneId::from(value)), value);
            assert_eq!(SessionId::from(value).as_u32(), value);
            assert_eq!(WindowId::from(value).as_u32(), value);
            assert_eq!(PaneId::from(value).as_u32(), value);
        }
    }

    #[test]
    fn identity_newtypes_are_serde_transparent() {
        assert_eq!(
            bincode::serialize(&PaneId::new(11)).expect("encodes"),
            bincode::serialize(&11_u32).expect("encodes")
        );
        assert_eq!(
            bincode::serialize(&WindowId::new(11)).expect("encodes"),
            bincode::serialize(&11_u32).expect("encodes")
        );
        assert_eq!(
            bincode::serialize(&SessionId::new(11)).expect("encodes"),
            bincode::serialize(&11_u32).expect("encodes")
        );
    }

    #[test]
    fn identity_id_newtypes_decode_back_through_serde() {
        for value in [0_u32, 7, 257, u32::MAX] {
            let session_bytes =
                bincode::serialize(&SessionId::new(value)).expect("session id encodes");
            let window_bytes =
                bincode::serialize(&WindowId::new(value)).expect("window id encodes");
            let pane_bytes = bincode::serialize(&PaneId::new(value)).expect("pane id encodes");

            assert_eq!(
                bincode::deserialize::<SessionId>(&session_bytes).expect("session id decodes"),
                SessionId::new(value),
            );
            assert_eq!(
                bincode::deserialize::<WindowId>(&window_bytes).expect("window id decodes"),
                WindowId::new(value),
            );
            assert_eq!(
                bincode::deserialize::<PaneId>(&pane_bytes).expect("pane id decodes"),
                PaneId::new(value),
            );
        }
    }

    #[test]
    fn identity_id_newtypes_total_order_matches_inner_u32() {
        let mut ids = [PaneId::new(3), PaneId::new(0), PaneId::new(1)];
        ids.sort();
        assert_eq!(ids, [PaneId::new(0), PaneId::new(1), PaneId::new(3)]);
    }

    #[test]
    fn session_name_already_sanitized_round_trips_through_serde() {
        let original = SessionName::new("alpha-beta_gamma").expect("printable name");
        let bytes = bincode::serialize(&original).expect("session name encodes");
        let restored: SessionName =
            bincode::deserialize(&bytes).expect("session name decodes idempotently");
        assert_eq!(restored, original);
    }
}
