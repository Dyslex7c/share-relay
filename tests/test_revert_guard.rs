use mev_share_relay::bundle::types::SearcherBundle;
use mev_share_relay::matchmaker::revert_guard::filter_failing_bundles;
use mev_share_relay::matchmaker::sim::ExecutionOutcome;

fn pair(id: u8, success: bool, user_balance_delta: i128) -> (SearcherBundle, ExecutionOutcome) {
    let bundle = SearcherBundle {
        bundle_id: [id; 32],
        items: Vec::new(),
        target_block: 1,
        min_timestamp: None,
        max_timestamp: None,
        kickback_bps: 9000,
    };
    (
        bundle,
        ExecutionOutcome {
            success,
            gas_used: 21_000,
            logs_emitted: Vec::new(),
            user_balance_delta,
            builder_payment: 0,
        },
    )
}

#[test]
fn only_successful_positive_outcomes_survive() {
    let kept = filter_failing_bundles(vec![
        pair(1, true, 100_000_000_000_000_000),
        pair(2, false, 100_000_000_000_000_000),
        pair(3, true, -50_000_000_000_000_000),
    ]);
    assert!(kept.len() == 1 && kept[0].bundle_id == [1; 32]);
}
