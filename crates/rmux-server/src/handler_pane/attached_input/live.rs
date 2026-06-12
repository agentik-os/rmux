use std::io;

use super::super::super::{prompt_support::PromptInputEvent, RequestHandler};
use super::super::io_other;
use super::super::pane_prompt_input::{
    decode_utf8_char, is_extended_key_prefix, is_utf8_lead_byte, utf8_expected_len,
};
use super::bracketed_paste::{decode_bracketed_paste, find_subslice, BracketedPasteDecode};
use super::kitty_graphics::{decode_kitty_graphics_apc, KittyGraphicsApcDecode};
use super::terminal_response::{decode_terminal_response, TerminalResponseDecode};
use super::{is_enter_key, is_mouse_prefix, retain_partial_attached_control_input};
use crate::input_keys::{decode_extended_key, decode_mouse, ExtendedKeyDecode, MouseDecode};
use crate::key_table::{decode_attached_key, AttachedKeyDecode, PREFIX_TABLE};

impl RequestHandler {
    #[async_recursion::async_recursion]
    pub(crate) async fn handle_attached_live_input(
        &self,
        attach_pid: u32,
        pending_input: &mut Vec<u8>,
        bytes: &[u8],
    ) -> io::Result<()> {
        self.handle_attached_live_input_inner(attach_pid, pending_input, bytes)
            .await
            .map(|_| ())
    }

    #[async_recursion::async_recursion]
    pub(crate) async fn handle_attached_live_input_inner(
        &self,
        attach_pid: u32,
        pending_input: &mut Vec<u8>,
        bytes: &[u8],
    ) -> io::Result<bool> {
        let mut forwarded_to_pane = false;
        let focused_window = {
            let session_name = {
                let active_attach = self.active_attach.lock().await;
                active_attach
                    .by_pid
                    .get(&attach_pid)
                    .map(|active| active.session_name.clone())
            };
            match session_name {
                Some(session_name) => {
                    let window_index = {
                        let state = self.state.lock().await;
                        state
                            .sessions
                            .session(&session_name)
                            .map(|session| session.active_window_index())
                    };
                    window_index.map(|window_index| (session_name, window_index))
                }
                None => None,
            }
        };
        if let Some((session_name, window_index)) = focused_window {
            let _ = self
                .clear_session_alerts_on_focus(&session_name, window_index)
                .await;
        }
        // Oversized-paste streaming: the opening `\x1b[200~` was already
        // forwarded by `flush_oversized_bracketed_paste`, so until the closing
        // `\x1b[201~` arrives EVERYTHING here is paste body — forward it
        // verbatim and never run it through the key/mouse decoders (a decoded
        // newline becomes Enter and submits mid-paste). A short tail is kept
        // buffered in case the closing marker is split across reads.
        if self.attached_paste_streaming(attach_pid).await {
            pending_input.extend_from_slice(bytes);
            if let Some(end) = find_subslice(pending_input, super::BRACKETED_PASTE_END) {
                let through_marker: Vec<u8> = pending_input
                    .drain(..end + super::BRACKETED_PASTE_END.len())
                    .collect();
                self.write_attached_bytes(attach_pid, &through_marker)
                    .await?;
                self.set_attached_paste_streaming(attach_pid, false).await;
                if pending_input.is_empty() {
                    return Ok(true);
                }
                // Bytes after the closing marker are ordinary live input again.
                let remaining = std::mem::take(pending_input);
                Box::pin(self.handle_attached_live_input_inner(
                    attach_pid,
                    pending_input,
                    &remaining,
                ))
                .await?;
                return Ok(true);
            }
            let keep = super::BRACKETED_PASTE_END.len().saturating_sub(1);
            if pending_input.len() <= keep {
                return Ok(false);
            }
            let flush_len = pending_input.len() - keep;
            let body: Vec<u8> = pending_input.drain(..flush_len).collect();
            self.write_attached_bytes(attach_pid, &body).await?;
            return Ok(true);
        }
        if self.prompt_active(attach_pid).await {
            self.handle_attached_prompt_input(attach_pid, pending_input, bytes)
                .await?;
            return Ok(false);
        }
        if self.mode_tree_active(attach_pid).await {
            self.handle_attached_mode_tree_input(attach_pid, pending_input, bytes)
                .await?;
            return Ok(false);
        }
        if self.overlay_active(attach_pid).await
            && self
                .handle_attached_overlay_input(attach_pid, pending_input, bytes)
                .await?
        {
            return Ok(false);
        }
        if self.display_panes_active(attach_pid).await {
            self.handle_attached_display_panes_input(attach_pid, pending_input, bytes)
                .await?;
            return Ok(false);
        }
        let target = self
            .attached_input_target(attach_pid)
            .await
            .map_err(io_other)?;
        if self
            .target_is_in_clock_mode(&target)
            .await
            .map_err(io_other)?
        {
            let _ = self.exit_clock_mode(&target).await.map_err(io_other)?;
            pending_input.clear();
            return Ok(false);
        }
        let target_in_copy_mode = self
            .target_is_in_copy_mode(&target)
            .await
            .map_err(io_other)?;

        pending_input.extend_from_slice(bytes);
        let backspace = self.attached_backspace_byte().await;
        let mut raw_start = 0;
        let mut offset = 0;

        while offset < pending_input.len() {
            let slice = &pending_input[offset..];
            match decode_bracketed_paste(slice) {
                BracketedPasteDecode::Matched { size } => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                    }
                    self.write_attached_bytes(attach_pid, &pending_input[offset..offset + size])
                        .await?;
                    forwarded_to_pane = true;
                    offset += size;
                    raw_start = offset;
                    continue;
                }
                BracketedPasteDecode::Partial => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                        forwarded_to_pane = true;
                    }
                    pending_input.drain(..offset);
                    // An unterminated bracketed paste that grows past the retain
                    // cap must NOT error out (that drops the whole paste and can
                    // kill the attach). Instead, flush the body so far verbatim
                    // to the PTY and keep only a short tail buffered in case the
                    // closing `\x1b[201~` marker is split across reads.
                    if self
                        .flush_oversized_bracketed_paste(attach_pid, pending_input)
                        .await?
                    {
                        forwarded_to_pane = true;
                    }
                    return Ok(forwarded_to_pane);
                }
                BracketedPasteDecode::NotPaste => {}
            }
            match decode_kitty_graphics_apc(slice) {
                KittyGraphicsApcDecode::Matched { size } => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                    }
                    self.write_attached_bytes(attach_pid, &pending_input[offset..offset + size])
                        .await?;
                    forwarded_to_pane = true;
                    offset += size;
                    raw_start = offset;
                    continue;
                }
                KittyGraphicsApcDecode::Partial => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                        forwarded_to_pane = true;
                    }
                    pending_input.drain(..offset);
                    retain_partial_attached_control_input(
                        "live kitty graphics APC",
                        pending_input,
                    )?;
                    return Ok(forwarded_to_pane);
                }
                KittyGraphicsApcDecode::NotKittyGraphics => {}
            }
            match decode_terminal_response(slice) {
                TerminalResponseDecode::Matched { size } => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                    }
                    self.write_attached_bytes(attach_pid, &pending_input[offset..offset + size])
                        .await?;
                    forwarded_to_pane = true;
                    offset += size;
                    raw_start = offset;
                    continue;
                }
                TerminalResponseDecode::Partial => {
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                        forwarded_to_pane = true;
                    }
                    pending_input.drain(..offset);
                    retain_partial_attached_control_input("live terminal response", pending_input)?;
                    return Ok(forwarded_to_pane);
                }
                TerminalResponseDecode::NotResponse => {}
            }
            if is_mouse_prefix(slice) {
                if raw_start < offset {
                    self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                        .await?;
                    forwarded_to_pane = true;
                }
                let last_mouse = self.attached_last_mouse_event(attach_pid).await;
                match decode_mouse(slice, last_mouse) {
                    MouseDecode::Matched { size, event } => {
                        self.handle_attached_live_mouse(attach_pid, event).await?;
                        offset += size;
                        raw_start = offset;
                    }
                    MouseDecode::Discard { size } => {
                        offset += size;
                        raw_start = offset;
                    }
                    MouseDecode::Partial => {
                        pending_input.drain(..raw_start);
                        retain_partial_attached_control_input("live mouse", pending_input)?;
                        return Ok(forwarded_to_pane);
                    }
                    MouseDecode::Invalid => {
                        raw_start = offset;
                        offset += 1;
                    }
                }
                continue;
            }
            if is_extended_key_prefix(slice) {
                if raw_start < offset {
                    self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                        .await?;
                    forwarded_to_pane = true;
                }
                match decode_extended_key(slice, backspace) {
                    ExtendedKeyDecode::Matched { size, key } => {
                        if raw_start < offset && is_enter_key(key) {
                            self.record_attached_submitted_text(
                                attach_pid,
                                &pending_input[raw_start..offset],
                            )
                            .await?;
                        }
                        if !self.handle_attached_live_key(attach_pid, key).await? {
                            forwarded_to_pane = true;
                        }
                        offset += size;
                        raw_start = offset;
                        if let Some(forwarded) = self
                            .reroute_attached_remaining_input_if_mode_changed(
                                attach_pid,
                                pending_input,
                                raw_start,
                            )
                            .await?
                        {
                            forwarded_to_pane |= forwarded;
                            return Ok(forwarded_to_pane);
                        }
                        if self.prompt_active(attach_pid).await {
                            break;
                        }
                        continue;
                    }
                    ExtendedKeyDecode::Partial => {
                        pending_input.drain(..raw_start);
                        retain_partial_attached_control_input("live extended key", pending_input)?;
                        return Ok(forwarded_to_pane);
                    }
                    ExtendedKeyDecode::Invalid => {}
                }
            }
            let prefix_table_active = self.attached_prefix_table_active(attach_pid).await;
            if slice
                .first()
                .is_some_and(|byte| byte.is_ascii() && !byte.is_ascii_control())
                && !prefix_table_active
                && !target_in_copy_mode
            {
                offset += 1;
                continue;
            }
            if !prefix_table_active
                && !target_in_copy_mode
                && slice.first().is_some_and(|byte| !byte.is_ascii())
            {
                if let Some((_, size)) = decode_utf8_char(slice) {
                    offset += size;
                    continue;
                }
                if slice.first().copied().is_some_and(is_utf8_lead_byte)
                    && slice.len()
                        < utf8_expected_len(
                            slice.first().copied().expect("slice has at least one byte"),
                        )
                {
                    pending_input.drain(..raw_start);
                    retain_partial_attached_control_input("live utf-8", pending_input)?;
                    return Ok(forwarded_to_pane);
                }
            }
            match decode_attached_key(slice, backspace) {
                AttachedKeyDecode::Matched { size, key } => {
                    if raw_start < offset && is_enter_key(key) {
                        self.record_attached_submitted_text(
                            attach_pid,
                            &pending_input[raw_start..offset],
                        )
                        .await?;
                    }
                    if raw_start < offset {
                        self.write_attached_bytes(attach_pid, &pending_input[raw_start..offset])
                            .await?;
                        forwarded_to_pane = true;
                    }
                    if !self.handle_attached_live_key(attach_pid, key).await? {
                        forwarded_to_pane = true;
                    }
                    offset += size;
                    raw_start = offset;
                    if let Some(forwarded) = self
                        .reroute_attached_remaining_input_if_mode_changed(
                            attach_pid,
                            pending_input,
                            raw_start,
                        )
                        .await?
                    {
                        forwarded_to_pane |= forwarded;
                        return Ok(forwarded_to_pane);
                    }
                    if self.prompt_active(attach_pid).await {
                        break;
                    }
                    continue;
                }
                AttachedKeyDecode::Partial => {
                    if target_in_copy_mode
                        && slice == b"\x1b"
                        && self
                            .handle_attached_copy_mode_key_event(
                                attach_pid,
                                target.clone(),
                                PromptInputEvent::Escape,
                            )
                            .await
                            .map_err(io_other)?
                    {
                        offset += 1;
                        raw_start = offset;
                        continue;
                    }
                    pending_input.drain(..raw_start);
                    retain_partial_attached_control_input("live attached key", pending_input)?;
                    return Ok(forwarded_to_pane);
                }
                AttachedKeyDecode::Invalid => {}
            }
            offset += 1;
        }

        if self.prompt_active(attach_pid).await && raw_start < pending_input.len() {
            let remaining = pending_input[raw_start..].to_vec();
            pending_input.clear();
            Box::pin(self.handle_attached_live_input(attach_pid, pending_input, &remaining))
                .await?;
            return Ok(forwarded_to_pane);
        }

        if raw_start < pending_input.len() {
            self.write_attached_bytes(attach_pid, &pending_input[raw_start..])
                .await?;
            forwarded_to_pane = true;
        }
        pending_input.clear();
        Ok(forwarded_to_pane)
    }

    async fn attached_prefix_table_active(&self, attach_pid: u32) -> bool {
        let active_attach = self.active_attach.lock().await;
        active_attach
            .by_pid
            .get(&attach_pid)
            .and_then(|active| active.key_table_name.as_deref())
            == Some(PREFIX_TABLE)
    }

    #[cfg(test)]
    pub(crate) async fn handle_attached_live_input_for_test(
        &self,
        attach_pid: u32,
        bytes: &[u8],
    ) -> io::Result<()> {
        let mut pending_input = Vec::new();
        self.handle_attached_live_input(attach_pid, &mut pending_input, bytes)
            .await
    }

    async fn reroute_attached_remaining_input_if_mode_changed(
        &self,
        attach_pid: u32,
        pending_input: &mut Vec<u8>,
        consumed: usize,
    ) -> io::Result<Option<bool>> {
        if consumed >= pending_input.len() {
            return Ok(None);
        }

        let target = self
            .attached_input_target(attach_pid)
            .await
            .map_err(io_other)?;
        let interactive_mode_active = self.prompt_active(attach_pid).await
            || self.mode_tree_active(attach_pid).await
            || self.overlay_active(attach_pid).await
            || self.display_panes_active(attach_pid).await
            || self
                .target_is_in_clock_mode(&target)
                .await
                .map_err(io_other)?;
        if !interactive_mode_active {
            return Ok(None);
        }

        let remaining = pending_input[consumed..].to_vec();
        pending_input.clear();
        let forwarded =
            Box::pin(self.handle_attached_live_input_inner(attach_pid, pending_input, &remaining))
                .await?;
        Ok(Some(forwarded))
    }
}
