use crate::matchmaker::{merger::merge_winning_bundles, result::SimulationResult};
use crate::searcher::bidder::calculate_bid_split;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MarketSimulation {
    pub searcher_count: usize,
    pub private_tx_count: usize,
    pub kickback_threshold_bps: u16,
}
pub fn run_synthetic_market_auction(sim: &MarketSimulation) -> (usize, u128, u128) {
    let mut candidates = Vec::new();
    let min_kb = sim.kickback_threshold_bps;
    for tx in 0..sim.private_tx_count {
        for s in 0..sim.searcher_count {
            let kb = (min_kb + (s as u16 % 5) * 100).min(9_500).max(min_kb);
            let b_bps = (10_000u16.saturating_sub(kb) * 8) / 10;
            let profit = ((tx + s + 1) as u128) * 100_000_000_000_000_000;
            if let Ok(sp) = calculate_bid_split(profit, kb, b_bps) {
                candidates.push(SimulationResult {
                    bundle_id: [(tx * sim.searcher_count + s) as u8; 32],
                    target_hash: [tx as u8; 32],
                    gas_used: 100_000,
                    bid_amount: sp.builder_payment + sp.user_kickback,
                    user_refund_amount: sp.user_kickback,
                    builder_payment: sp.builder_payment,
                });
            }
        }
    }
    let winners = merge_winning_bundles(&mut candidates);
    let b_sum = winners.iter().map(|w| w.builder_payment).sum();
    let u_sum = winners.iter().map(|w| w.user_refund_amount).sum();
    (winners.len(), b_sum, u_sum)
}
