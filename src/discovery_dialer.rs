// src/discovery_dialer.rs
use dashmap::DashSet;
use reth_network::NetworkHandle;
use reth_network_api::NetworkEventListenerProvider;
use reth::tasks::TaskExecutor;
use reth_network::PeerRequest;
use std::sync::Arc;
use std::time::Duration;
use futures_util::StreamExt;
use tracing::info;
use reth_network_api::Peers;
// use crate::conn_stats::ConnStats;

#[derive(Clone, Debug)]
pub struct DialPolicy {
    /// 同一 peer を何秒間再 dial しないか
    pub suppress_for: Duration,
    /// 1回 dial する間隔（レート制限）
    pub dial_interval: Duration,
}

impl Default for DialPolicy {
    fn default() -> Self {
        Self {
            suppress_for: Duration::from_secs(60),
            dial_interval: Duration::from_millis(300),
        }
    }
}

pub fn spawn_discovery_dialer(
    executor: &TaskExecutor,
    network: NetworkHandle,
    // stats: Arc<ConnStats>,
    policy: DialPolicy,
) {
    executor.spawn(Box::pin(async move {
        let mut discovery = network.discovery_listener();

        // 直近 suppress_for の間に dial した peer を覚える（簡易版: まずは “見たら一度だけ”）
        let seen = Arc::new(DashSet::new());

        let mut tick = tokio::time::interval(policy.dial_interval);

        loop {
            tokio::select! {
                _ = tick.tick() => {
                    // dial間隔の維持
                }
                ev = discovery.next() => {
                    let Some(ev) = ev else { break; };

                    // ここは reth の DiscoveryEvent / DiscoveredEvent の型に合わせて match を調整してください。
                    // あなたが以前参照していた形に合わせて書いています。
                    if let reth::network::DiscoveryEvent::NewNode(
                        reth::network::DiscoveredEvent::EventQueued { peer_id, addr, fork_id }
                    ) = ev {
                        let tcp = addr.tcp();

                        // active なら dial しない（実験ログが汚れないように）
                        // if stats.active_peers() > 0 && stats.active_peers() >= 64 {
                        //     warn!(target: "crawler::dialer", "Too many active peers; skipping dial");
                        //     continue;
                        // }

                        // “一度だけ” dial（まずこれで十分に原因が見える）
                        if !seen.insert(peer_id) {
                            continue;
                        }

                        info!(
                            target: "crawler::dialer",
                            %peer_id,
                            %tcp,
                            ?fork_id,
                            "discovered; dialing outbound (TCP -> RLPx)"
                        );

                        network.connect_peer(peer_id, tcp);
                    }
                }
            }
        }
    }));
}
