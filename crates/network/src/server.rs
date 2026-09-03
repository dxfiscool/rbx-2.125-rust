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

/// `RBX::RunningAverage<double,double>` (IDA 0x2a60b0): exponential moving
/// average with variance tracking. `alpha` is the blend weight at +8, `mean`
/// at +24, `variance` at +32; `fresh` (+40) seeds the mean with the first
/// sample. The optional history buffer at +0 is not modeled; the
/// mean/variance state the stats sites read is fully preserved.
#[derive(Clone, Copy, Debug)]
pub struct RunningAverage {
    pub alpha: f64,
    pub last: f64,
    pub mean: f64,
    pub variance: f64,
    pub fresh: bool,
}

impl Default for RunningAverage {
    fn default() -> Self {
        Self {
            alpha: 0.0,
            last: 0.0,
            mean: 0.0,
            variance: 0.0,
            fresh: true,
        }
    }
}

impl RunningAverage {
    pub fn with_alpha(alpha: f64) -> Self {
        Self {
            alpha,
            ..Default::default()
        }
    }

    /// `RBX::RunningAverage<double,double>::sample` (IDA 0x2a60b0).
    pub fn sample(&mut self, value: f64) {
        // IDA 0x2a60e0: infinite samples are dropped.
        if value.is_infinite() {
            return;
        }
        // IDA 0x2a60e6..0x2a6106: first sample seeds, later ones blend.
        let blended = if self.fresh {
            value
        } else {
            self.alpha.mul_add(value, (1.0 - self.alpha) * self.mean)
        };
        self.mean = blended; // IDA 0x2a610c
        self.last = value; // IDA 0x2a6110
        self.fresh = false; // IDA 0x2a6114
        // IDA 0x2a6140: variance tracks the squared deviation.
        let dev = value - self.mean;
        self.variance = (1.0 - self.alpha).mul_add(self.variance, self.alpha * dev * dev);
    }
}

/// `RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats`:
/// per-address snapshot refreshed by `updateStats`.
#[derive(Clone, Debug, Default)]
pub struct ConnectionStats {
    /// `a2[5]`: connection-state query result (IDA 0x99f462, vtable +224).
    pub state: i32,
    /// `a2[6]`: last ping (IDA 0x99f4a6, vtable +164).
    pub last_ping_ms: i32,
    /// `a2[7]`: average ping (IDA 0x99f4e0, vtable +168).
    pub average_ping_ms: i32,
    /// `a2[8]`: lowest ping (IDA 0x99f51a, vtable +172).
    pub lowest_ping_ms: i32,
    /// 212-byte `RakNetStatistics` block copied when present
    /// (IDA 0x99f540 `memcpy(..., 0xD4)`).
    pub raknet_block: Option<[u8; 0xD4]>,
    /// `a2 + 62`: samples `stats[132] != 0` as 1.0/0.0 (IDA 0x99f548..0x99f564).
    pub loss_average: RunningAverage,
    /// `a2 + 73`: samples `stats[120] != 0` as 1.0/0.0 (IDA 0x99f568..0x99f57c).
    pub activity_average: RunningAverage,
}

/// One `RakPeerInterface` poll for an address: the four scalar queries plus
/// the optional statistics block with its two flag bytes.
#[derive(Clone, Debug)]
pub struct PeerSnapshot {
    pub state: i32,
    pub last_ping_ms: i32,
    pub average_ping_ms: i32,
    pub lowest_ping_ms: i32,
    pub stats: Option<PeerRakNetStats>,
}

/// The statistics reply: raw block plus the sampled flag bytes.
#[derive(Clone, Debug)]
pub struct PeerRakNetStats {
    pub block: [u8; 0xD4],
    pub flag_120: bool,
    pub flag_132: bool,
}

impl ConnectionStats {
    /// `RBX::Network::ConcurrentRakPeer::StatsUpdateJob::updateStats`
    /// (IDA 0x99f428). The declared `int` return is a decompiler artifact of
    /// the stack guard (`return __stack_chk_guard...`, IDA 0x99f59a); the
    /// function only writes through `a2`, so this returns `()`.
    pub fn update_stats(&mut self, snap: &PeerSnapshot) {
        self.state = snap.state; // IDA 0x99f462
        self.last_ping_ms = snap.last_ping_ms; // IDA 0x99f4a6
        self.average_ping_ms = snap.average_ping_ms; // IDA 0x99f4e0
        self.lowest_ping_ms = snap.lowest_ping_ms; // IDA 0x99f51a
        // IDA 0x99f532..0x99f536: no statistics block => done.
        let Some(stats) = &snap.stats else {
            return;
        };
        self.raknet_block = Some(stats.block); // IDA 0x99f540
        // IDA 0x99f548..0x99f55c: `v8[132] ? 1.0 : 0.0`.
        self.loss_average.sample(f64::from(stats.flag_132 as u8));
        // IDA 0x99f568..0x99f574: `v8[120] ? 1.0 : 0.0`.
        self.activity_average.sample(f64::from(stats.flag_120 as u8));
    }
}

/// `RBX::Network::Server` (relevant slice): the nullable player list at
/// +0x38 (IDA 0x9c72ae..0x9c72b8), the bound port at +684 (IDA 0x9c6e90),
/// and liveness for the `IsActive` gate in `stop` (IDA 0x9c728e).
#[derive(Clone, Debug, Default)]
pub struct Server {
    pub players: Option<Vec<SharedPtr<ServerClient>>>,
    pub port: Option<u16>,
    pub active: bool,
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

    /// `RBX::Network::Server::start` (IDA 0x9c6da4). `startup` is the outcome
    /// of `rawPeer->Startup(128, SocketDescriptor(port), ...)` (IDA 0x9c6e64):
    /// `Ok(bound)` carries the `GetPort` result stored at +684
    /// (IDA 0x9c6e90), `Err(code)` the nonzero return that throws
    /// `std::runtime_error("Failed to start network server, id %d")`
    /// (IDA 0x9c704e..0x9c7094), mirrored here as `Err(String)`.
    /// The `DebugDisableTimeoutDisconnect` 600-second override (IDA 0x9c6fa2)
    /// lives on the peer handle and is not modeled.
    pub fn start(&mut self, startup: Result<u16, i32>) -> Result<u16, String> {
        match startup {
            Err(code) => Err(format!("Failed to start network server, id {code}")),
            Ok(bound) => {
                self.port = Some(bound);
                self.active = true;
                Ok(bound)
            }
        }
    }

    /// `RBX::Network::Server::stop` (IDA 0x9c7234): unlocks each child
    /// (IDA 0x9c7274 `visitChildren` with `unlockParent`), drops them all
    /// (IDA 0x9c727a `removeAllChildren`), then disconnects with the block
    /// duration (IDA 0x9c7294..0x9c72a2 `Shutdown(a2, 0, 3)`) when the peer
    /// is active (IDA 0x9c728e). Returns whether a shutdown ran.
    pub fn stop(&mut self, block_duration_ms: i32) -> bool {
        // IDA 0x9c725a: `FLog "NetworkServer:Stop blockDuration(%d)"`.
        let _ = block_duration_ms;
        self.players = None;
        if self.active {
            self.active = false;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_list_counts_zero() {
        assert_eq!(
            Server {
                players: None,
                ..Default::default()
            }
            .client_count(),
            0
        );
    }

    #[test]
    fn counts_only_replicators() {
        let server = Server {
            players: Some(vec![
                SharedPtr::new(ServerClient { is_replicator: true }),
                SharedPtr::new(ServerClient { is_replicator: false }),
                SharedPtr::new(ServerClient { is_replicator: true }),
            ]),
            ..Default::default()
        };
        assert_eq!(server.client_count(), 2);
    }

    #[test]
    fn start_stores_bound_port() {
        let mut server = Server::default();
        assert_eq!(server.start(Ok(53640)), Ok(53640));
        assert_eq!(server.port, Some(53640));
        assert!(server.active);
        assert!(server.stop(3));
        assert!(!server.active);
        assert_eq!(server.port, Some(53640));
        assert!(!server.stop(3));
    }

    #[test]
    fn start_failure_reports_id() {
        let mut server = Server::default();
        assert_eq!(
            server.start(Err(7)),
            Err("Failed to start network server, id 7".to_owned())
        );
        assert!(!server.active);
    }

    #[test]
    fn stop_clears_players() {
        let mut server = Server {
            players: Some(vec![SharedPtr::new(ServerClient {
                is_replicator: true,
            })]),
            active: true,
            ..Default::default()
        };
        assert!(server.stop(0));
        assert_eq!(server.client_count(), 0);
    }

    #[test]
    fn update_stats_copies_block_and_samples_flags() {
        let mut stats = ConnectionStats {
            loss_average: RunningAverage::with_alpha(0.5),
            activity_average: RunningAverage::with_alpha(0.5),
            ..Default::default()
        };
        let snap = PeerSnapshot {
            state: 3,
            last_ping_ms: 11,
            average_ping_ms: 12,
            lowest_ping_ms: 9,
            stats: Some(PeerRakNetStats {
                block: [7u8; 0xD4],
                flag_120: true,
                flag_132: false,
            }),
        };
        stats.update_stats(&snap);
        assert_eq!(stats.state, 3);
        assert_eq!(stats.last_ping_ms, 11);
        assert_eq!(stats.raknet_block, Some([7u8; 0xD4]));
        // First samples seed the means: loss 0.0, activity 1.0.
        assert_eq!(stats.loss_average.mean, 0.0);
        assert_eq!(stats.activity_average.mean, 1.0);
        // Missing block refreshes scalars but leaves the block alone.
        let quiet = PeerSnapshot { stats: None, ..snap };
        stats.update_stats(&quiet);
        assert_eq!(stats.raknet_block, Some([7u8; 0xD4]));
    }

    #[test]
    fn running_average_blends_and_drops_infinity() {
        let mut avg = RunningAverage::with_alpha(0.5);
        avg.sample(4.0);
        assert_eq!(avg.mean, 4.0);
        avg.sample(8.0);
        assert_eq!(avg.mean, 6.0);
        avg.sample(f64::INFINITY);
        assert_eq!(avg.mean, 6.0);
    }
}
