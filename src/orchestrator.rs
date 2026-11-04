use crate::bundle::searcher::SearcherRegistry;
use crate::hint::sse::SseBroadcaster;
use crate::matchmaker::kickback::KickbackLedger;
use crate::privacy::replay::ReplayFilter;
const SSE_CHANNEL_CAPACITY: usize = 1024;
pub struct MatchmakerEngine {
    pub broadcaster: SseBroadcaster,
    pub searchers: SearcherRegistry,
    pub replay_filter: ReplayFilter,
    pub kickbacks: KickbackLedger,
}
impl Default for MatchmakerEngine {
    fn default() -> Self {
        Self::new()
    }
}
impl MatchmakerEngine {
    pub fn new() -> Self {
        Self {
            broadcaster: SseBroadcaster::new(SSE_CHANNEL_CAPACITY),
            searchers: SearcherRegistry::default(),
            replay_filter: ReplayFilter::default(),
            kickbacks: KickbackLedger::default(),
        }
    }
    pub fn process_incoming_tx(
        &self,
        tx: crate::hint::tx::PrivateTransaction,
    ) -> Result<crate::hint::sanitizer::SanitizedEvent, &'static str> {
        let event = crate::hint::sanitizer::sanitize_transaction(&tx, &[]);
        self.broadcaster.publish(event.clone()).map(|_| event)
    }
}
