use mev_share_relay::searcher::bidder::calculate_bid_split;
#[test]
fn five_searchers_escalate_until_builder_budget_is_exhausted() {
    const PROFIT: u128 = 1_000_000_000_000_000_000;
    let mut bids = [PROFIT / 100; 5];
    let (mut builder_bps, mut highest, mut rounds) = (100u16, PROFIT / 100, 0);
    loop {
        let previous_round = highest;
        for bid in &mut bids {
            let next_bps = (builder_bps + builder_bps.div_ceil(20)).min(1000);
            let split = calculate_bid_split(PROFIT, 9000, next_bps).expect("protected bid");
            assert!(split.user_kickback >= PROFIT * 9 / 10);
            let total = split.user_kickback + split.builder_payment + split.searcher_profit;
            assert_eq!(total, PROFIT);
            assert!(split.builder_payment >= highest);
            assert!(builder_bps == 1000 || split.builder_payment > highest);
            *bid = split.builder_payment;
            highest = *bid;
            builder_bps = next_bps;
        }
        rounds += 1;
        if highest == previous_round {
            break;
        }
        assert!(rounds < 100, "bidding failed to converge");
    }
    assert!(rounds > 1);
    assert_eq!(bids, [PROFIT / 10; 5]);
    let final_split = calculate_bid_split(PROFIT, 9000, builder_bps).expect("converged bid");
    assert_eq!(final_split.searcher_profit, 0);
    let next_bps = builder_bps + builder_bps.div_ceil(20);
    assert!(calculate_bid_split(PROFIT, 9000, next_bps).is_err());
}
