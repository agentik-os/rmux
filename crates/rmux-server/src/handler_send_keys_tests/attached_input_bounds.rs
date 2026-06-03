use super::*;

async fn create_attached_live_session(
    handler: &RequestHandler,
    name: &rmux_proto::SessionName,
    requester_pid: u32,
) -> mpsc::UnboundedReceiver<crate::pane_io::AttachControl> {
    #[cfg(unix)]
    {
        let mut state = handler.state.lock().await;
        state
            .options
            .set(
                ScopeSelector::Global,
                OptionName::DefaultShell,
                "/bin/bash".to_owned(),
                SetOptionMode::Replace,
            )
            .expect("test default-shell is valid");
    }

    let created = handler
        .handle(Request::NewSession(NewSessionRequest {
            session_name: name.clone(),
            detached: true,
            size: Some(TerminalSize { cols: 80, rows: 24 }),
            environment: None,
        }))
        .await;
    assert!(matches!(created, Response::NewSession(_)));

    let (control_tx, control_rx) = mpsc::unbounded_channel();
    let _attach_id = handler
        .register_attach(requester_pid, name.clone(), control_tx)
        .await;
    control_rx
}

#[tokio::test]
async fn live_attach_unterminated_bracketed_paste_flushes_body_without_error() {
    // An unterminated bracketed paste that grows past the retain cap must NOT
    // error out (that would drop the whole paste and could kill the attach).
    // Instead the leading `\x1b[200~` start marker is stripped and the paste
    // body is flushed verbatim to the pane, retaining only a short tail in case
    // the closing `\x1b[201~` is split across reads.
    const PASTE_END_LEN: usize = b"\x1b[201~".len();
    const PASTE_START_LEN: usize = b"\x1b[200~".len();

    let handler = RequestHandler::new();
    let alpha = session_name("alpha");
    let requester_pid = std::process::id();
    let _control_rx = create_attached_live_session(&handler, &alpha, requester_pid).await;

    let capture_target = {
        let target = PaneTarget::new(alpha.clone(), 0);
        let state = handler.state.lock().await;
        state.start_pane_input_capture_for_test(&target);
        target
    };

    let mut pending_input = Vec::new();
    handler
        .handle_attached_live_input(requester_pid, &mut pending_input, b"\x1b[200~")
        .await
        .expect("bracketed paste start is retained");
    assert_eq!(pending_input, b"\x1b[200~");

    // Push the still-unterminated paste one byte past the retain cap.
    let overflow = vec![b'a'; DEFAULT_MAX_FRAME_LENGTH - pending_input.len() + 1];
    handler
        .handle_attached_live_input(requester_pid, &mut pending_input, &overflow)
        .await
        .expect("oversized unterminated bracketed paste flushes instead of erroring");

    // Tail retained for a possibly-split closing marker: end-marker length - 1.
    let retained_tail = PASTE_END_LEN - 1;
    assert_eq!(pending_input.len(), retained_tail);
    assert!(pending_input.iter().all(|byte| *byte == b'a'));

    // Body flushed verbatim, start marker stripped, only the tail withheld.
    let total_body = overflow.len(); // start marker is not part of the body
    let _ = PASTE_START_LEN;
    let expected_flushed = vec![b'a'; total_body - retained_tail];
    let state = handler.state.lock().await;
    assert_eq!(
        state.pane_input_capture_for_test(&capture_target),
        Some(expected_flushed)
    );
}

#[tokio::test]
async fn live_attach_chunked_sgr_mouse_sequence_still_dispatches() {
    let handler = RequestHandler::new();
    let alpha = session_name("alpha");
    let requester_pid = std::process::id();
    let _control_rx = create_attached_live_session(&handler, &alpha, requester_pid).await;

    {
        let mut state = handler.state.lock().await;
        state
            .append_bytes_to_pane_transcript_for_test(&alpha, 0, 0, b"\x1b[?1003h\x1b[?1006h")
            .expect("mouse any and sgr transcript update");
    }

    #[cfg(windows)]
    let expected = encode_mouse_event(
        mode::MODE_MOUSE_ALL | mode::MODE_MOUSE_SGR,
        &MouseForwardEvent {
            b: 64,
            lb: 0,
            x: 1,
            y: 1,
            lx: 0,
            ly: 0,
            sgr_b: 64,
            sgr_type: 'M',
            ignore: false,
        },
        1,
        1,
    )
    .expect("sgr wheel encodes");

    #[cfg(windows)]
    let capture = RawPaneInputProbe::start(
        &handler,
        &alpha,
        "live-attach-chunked-sgr-wheel",
        expected.len(),
    )
    .await;

    let mut pending_input = Vec::new();
    handler
        .handle_attached_live_input(requester_pid, &mut pending_input, b"\x1b[<64;2")
        .await
        .expect("first sgr mouse chunk");
    assert_eq!(pending_input, b"\x1b[<64;2");

    handler
        .handle_attached_live_input(requester_pid, &mut pending_input, b";2M")
        .await
        .expect("second sgr mouse chunk");
    assert!(pending_input.is_empty());

    let active_attach = handler.active_attach.lock().await;
    let event = active_attach
        .by_pid
        .get(&requester_pid)
        .and_then(|active| active.mouse.current_event.as_ref())
        .expect("current chunked wheel event");
    assert_eq!(event.location, MouseLocation::Pane);
    assert_eq!(event.raw.b, 64);
    drop(active_attach);

    #[cfg(windows)]
    {
        capture.finish(&handler, &alpha).await;
        capture.assert_contents(&handler, &expected).await;
    }
}

#[tokio::test]
async fn live_attach_unterminated_sgr_mouse_is_bounded_without_pane_leak() {
    let handler = RequestHandler::new();
    let alpha = session_name("alpha");
    let requester_pid = std::process::id();
    let _control_rx = create_attached_live_session(&handler, &alpha, requester_pid).await;

    #[cfg(windows)]
    let capture_target = {
        let target = PaneTarget::new(alpha.clone(), 0);
        let state = handler.state.lock().await;
        state.start_pane_input_capture_for_test(&target);
        target
    };

    let mut pending_input = Vec::new();
    handler
        .handle_attached_live_input(requester_pid, &mut pending_input, b"\x1b[<")
        .await
        .expect("sgr mouse start is retained");
    assert_eq!(pending_input, b"\x1b[<");

    let overflow = vec![b'9'; DEFAULT_MAX_FRAME_LENGTH - pending_input.len() + 1];
    let err = handler
        .handle_attached_live_input(requester_pid, &mut pending_input, &overflow)
        .await
        .expect_err("unterminated sgr mouse should be bounded");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("live mouse"));
    assert!(pending_input.is_empty());

    #[cfg(windows)]
    {
        let state = handler.state.lock().await;
        assert_eq!(
            state.pane_input_capture_for_test(&capture_target),
            Some(Vec::new())
        );
    }
}
