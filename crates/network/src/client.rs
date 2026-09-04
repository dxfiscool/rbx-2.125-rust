//! `RBX::Network::Client` packet handling.
//!
//! Decompiled from `Client::OnFailedConnectionAttempt` (IDA 0x968fb0),
//! `Client::OnReceive` (IDA 0x969704), and
//! `RakNet::PluginInterface2::OnReceive` (IDA 0x96d260). Packet bytes,
//! logging, and signal plumbing stay engine-side; the dispatch gates and
//! message texts live here.

#![allow(dead_code)]

/// Connection-failure message by packet code (IDA 0x969004..0x96904e,
/// repeated at 0x96990c..0x96a3b4): 17 is a plain failure, 24 is an
/// out-of-date client, anything else formats the code.
#[must_use]
pub fn connection_error_text(code: u8) -> String {
    match code {
        17 => "Connection attempt failed".to_owned(),
        24 => "Roblox version is out of date. Please upgrade.".to_owned(),
        _ => format!("Network error {code}"),
    }
}

/// `Client::OnFailedConnectionAttempt` (IDA 0x968fb0) and its non-virtual
/// thunk (IDA 0x9694b4): logs `Failed to connect to <addr>. <msg>` and
/// fires the `(address, code, message)` signal engine-side.
pub fn on_failed_connection_attempt(code: u8, fire: &mut dyn FnMut(u8, String)) {
    fire(code, connection_error_text(code));
}

/// `Client::OnReceive` packet dispatch (IDA 0x96974a..0x96992e) and its
/// non-virtual thunk (IDA 0x96c474): 21/22 reset the server id, 24 runs
/// the invalid-password path (which reuses [`connection_error_text`]),
/// 16 runs the connection-accepted setup (ticket, spawn name, replicator,
/// terrain clear, CRC threads, connected signal), and everything else is
/// ignored. Always returns 1.
pub fn client_on_receive(
    kind: u8,
    code: u8,
    disconnect: &mut dyn FnMut(),
    invalid_password: &mut dyn FnMut(u8),
    accepted: &mut dyn FnMut(),
) -> u32 {
    match kind {
        21 | 22 => disconnect(),
        24 => invalid_password(code),
        16 => accepted(),
        _ => {}
    }
    1
}

/// `RakNet::PluginInterface2::OnReceive` (IDA 0x96d260): the base
/// implementation just continues processing.
#[must_use]
pub fn plugin_on_receive() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_texts_match_original() {
        assert_eq!(connection_error_text(17), "Connection attempt failed");
        assert_eq!(
            connection_error_text(24),
            "Roblox version is out of date. Please upgrade."
        );
        assert_eq!(connection_error_text(7), "Network error 7");
    }

    #[test]
    fn receive_dispatch_gates() {
        // IDA 0x968fb0/0x969704: failure fires code+text, dispatch by kind.
        let mut log = Vec::new();
        on_failed_connection_attempt(24, &mut |code, text| log.push((code, text)));
        assert_eq!(
            log,
            [(24, "Roblox version is out of date. Please upgrade.".to_owned())]
        );
        let seen = std::cell::RefCell::new(Vec::new());
        let recv = |kind: u8, code: u8| {
            client_on_receive(
                kind,
                code,
                &mut || seen.borrow_mut().push("dc"),
                &mut |c| seen.borrow_mut().push(if c == 17 { "pw17" } else { "pw" }),
                &mut || seen.borrow_mut().push("ok"),
            )
        };
        assert_eq!(recv(21, 0), 1);
        assert_eq!(recv(22, 0), 1);
        assert_eq!(recv(24, 17), 1);
        assert_eq!(recv(16, 0), 1);
        assert_eq!(recv(99, 0), 1);
        assert_eq!(seen.borrow().as_slice(), ["dc", "dc", "pw17", "ok"]);
        assert_eq!(plugin_on_receive(), 1);
    }
}
