use mev_share_relay::matchmaker::merger::merge_winning_bundles;
use mev_share_relay::matchmaker::result::SimulationResult;
use std::time::{Duration, Instant};
#[test]
fn resolves_500_bids_across_50_targets_quickly() {
    let mut candidates: Vec<SimulationResult> = (0..500_u32)
        .map(|i| {
            let target = i % 50;
            SimulationResult {
                bundle_id: [i as u8; 32],
                target_hash: [target as u8; 32],
                gas_used: 100_000,
                bid_amount: u128::from(i) * 1_000,
                user_refund_amount: u128::from(i) * 900,
                builder_payment: u128::from(i) * 100,
            }
        })
        .collect();
    let started = Instant::now();
    let winners = merge_winning_bundles(&mut candidates);
    let elapsed = started.elapsed();
    assert_eq!(winners.len(), 50);
    assert!(
        elapsed < Duration::from_millis(20),
        "resolution took {elapsed:?}"
    );
    let mut targets: Vec<[u8; 32]> = winners.iter().map(|w| w.target_hash).collect();
    targets.sort_unstable();
    targets.dedup();
    assert_eq!(targets.len(), 50);
    let expected: Vec<u128> = (450u32..500).rev().map(|b| u128::from(b) * 1_000).collect();
    let bids: Vec<u128> = winners.iter().map(|w| w.bid_amount).collect();
    assert_eq!(bids, expected);
}
