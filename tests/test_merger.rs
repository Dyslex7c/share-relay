use mev_share_relay::matchmaker::merger::merge_winning_bundles;
use mev_share_relay::matchmaker::result::SimulationResult;

fn candidate(searcher: u8, target: u8, bid_amount: u128) -> SimulationResult {
    SimulationResult {
        bundle_id: [searcher; 32],
        target_hash: [target; 32],
        gas_used: 100_000,
        bid_amount,
        user_refund_amount: 0,
        builder_payment: bid_amount,
    }
}

#[test]
fn higher_bid_wins_conflict_without_discarding_other_target() {
    let searcher_one = candidate(1, 10, 300_000);
    let searcher_two = candidate(2, 10, 200_000);
    let searcher_three = candidate(3, 20, 100_000);
    let mut candidates = [
        searcher_two.clone(),
        searcher_three.clone(),
        searcher_one.clone(),
    ];
    let winners = merge_winning_bundles(&mut candidates);
    assert_eq!(winners, vec![searcher_one, searcher_three]);
    assert!(!winners.contains(&searcher_two));
}
