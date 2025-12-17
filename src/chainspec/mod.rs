pub mod hl;
use alloy_chains::{Chain, NamedChain};

use reth::chainspec::ChainSpec;
use std::sync::LazyLock;

static HL_CHAINSPEC: LazyLock<ChainSpec> = LazyLock::new(|| hl::hl_chainspec(Chain::from_named(NamedChain::Hyperliquid), include_str!("genesis.json")));
