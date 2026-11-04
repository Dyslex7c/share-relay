use mev_share_relay::matchmaker::{merger::merge_winning_bundles, result::SimulationResult};
use std::sync::mpsc;
use std::time::{Duration, Instant};
const TARGET: [u8; 32] = [0x5a; 32];
#[test]
fn hundred_concurrent_searchers_race_to_one_winner_under_five_ms() {
    let (tx, rx) = mpsc::channel();
    std::thread::scope(|scope| {
        for i in 0..100u32 {
            let tx = tx.clone();
            scope.spawn(move || {
                let bid = (u128::from(i) + 1) * 10u128.pow(18);
                let result = SimulationResult {
                    bundle_id: [i as u8; 32],
                    target_hash: TARGET,
                    gas_used: 100_000 + u64::from(i),
                    bid_amount: bid,
                    user_refund_amount: bid / 10,
                    builder_payment: bid / 5,
                };
                tx.send(result).expect("auction still open");
            });
        }
    });
    let mut candidates: Vec<SimulationResult> = rx.iter().take(100).collect();
    assert_eq!(candidates.len(), 100);
    let started = Instant::now();
    let winners = merge_winning_bundles(&mut candidates);
    let elapsed = started.elapsed();
    assert!(elapsed < Duration::from_millis(5), "took {elapsed:?}");
    assert_eq!((winners.len(), winners[0].bundle_id), (1, [99; 32]));
}
