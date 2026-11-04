// Sources: [S12] this crate `src/searcher/listener.rs:10-19` (filter gate) + `assembler.rs`
// (bundle shape) — verified. 9000 bps is task-specified; mev-share's real default is `unknown` and
// v0.1 wires refunds as integer percent. `raw` is a placeholder, not a signed tx: no RLP encoder.
use super::assembler::assemble_backrun_bundle;
use super::client::SearcherRpcClient;
use super::listener::{should_process_event, HintFilter};
use super::pool::AmmPool;
use crate::bundle::types::SearcherBundle;
use crate::hint::sanitizer::SanitizedEvent;
type Bundle = SearcherBundle;
pub const DEFAULT_KICKBACK_BPS: u16 = 9_000;
#[derive(Clone, Debug, Default)]
pub struct SearcherBot {
    pub filter: HintFilter,
    pub client: SearcherRpcClient,
    pub pools: Vec<AmmPool>,
}
impl SearcherBot {
    pub fn new(filter: HintFilter, client: SearcherRpcClient) -> Self {
        let mut bot = Self::default();
        (bot.filter, bot.client) = (filter, client);
        bot
    }
    pub fn handle_hint_event(&self, event: &SanitizedEvent, target_block: u64) -> Option<Bundle> {
        if !should_process_event(&self.filter, event) {
            return None;
        }
        let hash = event.hash?;
        let kickback = DEFAULT_KICKBACK_BPS;
        let pool = self.pools.first().map(|p| p.pool_address.to_vec());
        let raw = [pool.unwrap_or_default(), hash.to_vec()].concat();
        Some(assemble_backrun_bundle(hash, raw, target_block, kickback))
    }
}
