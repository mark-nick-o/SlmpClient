use crate::{SLMPClearMode, SLMPCommand, SLMPConnectionInfo};

pub fn send_remote_run_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    force: bool,
    clear_mode: SLMPClearMode,
) -> u16 {
    let mut buf = Vec::new();
    match force {
        true => buf.push(0x03),
        false => buf.push(0x01),
    }
    buf.push(0);
    buf.push(clear_mode as u8);
    buf.push(0);

    connection_info.send_cmd(timeout, SLMPCommand::RemoteRun, 0, &buf)
}

pub fn send_remote_stop_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> u16 {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteStop, 0, &[1u8, 0])
}
pub fn send_remote_pause_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    force: bool,
) -> u16 {
    let buf = match force {
        true => [1, 0],
        false => [3, 0],
    };
    connection_info.send_cmd(timeout, SLMPCommand::RemotePause, 0, &buf)
}
pub fn send_remote_latch_clear(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> u16 {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteLatchClear, 0, &[1, 0])
}
pub fn send_remote_reset_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> u16 {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteReset, 0, &[1, 0])
}
pub fn send_read_type_name_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> u16 {
    connection_info.send_cmd(timeout, SLMPCommand::ReadTypeName, 0, &[])
}
pub fn decode_read_type_name_response(buf: &[u8]) -> (String, u16) {
    let mut s = String::new();
    for i in 0..16 {
        s.push(buf[i] as char);
    }
    let code = buf[16] as u16 + (buf[17] as u16) << 8;
    (s, code)
}
