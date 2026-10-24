#[cfg(test)]
#[cfg(feature = "web_server_capability")]
mod terminal_loop_tests;

#[test]
fn toggle_mouse_mode_message_maps_to_client_instruction() {
    use crate::ClientInstruction;
    use zellij_utils::ipc::ServerToClientMsg;

    assert!(matches!(
        ClientInstruction::from(ServerToClientMsg::ToggleMouseMode),
        ClientInstruction::ToggleMouseMode
    ));
}

#[cfg(test)]
mod teardown_tests;

#[cfg(test)]
mod ipc_pipe_length_tests;

#[cfg(test)]
mod cli_client_tests;

#[cfg(test)]
mod input_handler_tests;
