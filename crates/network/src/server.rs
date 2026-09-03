//! `RBX::Network::Server` client census.
//!
//! Decompiled from 0x9c72a8 (`Server::getClientCount`): when the player
//! list at +0x38 is null the count is 0, otherwise it counts entries
//! satisfying the `isReplicator` predicate (IDA 0x9c72be..
//! `std::count_if` at 0x9c72c8).

#![allow(dead_code)]

use rbx_core::SharedPtr;

/// One entry of the server player list (IDA 0x9c72ba: a
/// `vector<shared_ptr<Instance>>` element).
#[derive(Clone, Debug, Default)]
pub struct ServerClient {
    /// Whether `isReplicator(shared_ptr<Instance>)` holds for this entry.
    pub is_replicator: bool,
}

fn is_replicator(client: &SharedPtr<ServerClient>) -> bool {
    // IDA `__ZL12isReplicatorN5boost10shared_ptrIN3RBX8InstanceEEE`.
    client.is_replicator
}

/// `RBX::Network::Server` (relevant slice): the nullable player list at
/// +0x38 (IDA 0x9c72ae..0x9c72b8).
#[derive(Clone, Debug, Default)]
pub struct Server {
    pub players: Option<Vec<SharedPtr<ServerClient>>>,
}

impl Server {
    /// `RBX::Network::Server::getClientCount` (IDA 0x9c72a8).
    pub fn client_count(&self) -> usize {
        // IDA 0x9c72b2..0x9c72b6: null list => 0.
        let Some(players) = &self.players else {
            return 0;
        };
        // IDA 0x9c72ba..0x9c72c8: `std::count_if` with `isReplicator`.
        players.iter().filter(|c| is_replicator(c)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_list_counts_zero() {
        assert_eq!(Server { players: None }.client_count(), 0);
    }

    #[test]
    fn counts_only_replicators() {
        let server = Server {
            players: Some(vec![
                SharedPtr::new(ServerClient { is_replicator: true }),
                SharedPtr::new(ServerClient { is_replicator: false }),
                SharedPtr::new(ServerClient { is_replicator: true }),
            ]),
        };
        assert_eq!(server.client_count(), 2);
    }
}
