use mev_share_relay::searcher::bidder::calculate_bid_split;

const E18: u128 = 1_000_000_000_000_000_000;

#[test]
fn ninety_percent_kickback_eight_percent_builder_split() {
    let split = calculate_bid_split(E18, 9000, 800).expect("valid bps");
    assert_eq!(split.user_kickback, 900_000_000_000_000_000);
    assert_eq!(split.builder_payment, 80_000_000_000_000_000);
    assert_eq!(split.searcher_profit, 20_000_000_000_000_000);
    let total = split.user_kickback + split.builder_payment + split.searcher_profit;
    assert_eq!(total, E18);
}

#[test]
fn bps_sum_above_one_hundred_percent_is_rejected() {
    assert!(calculate_bid_split(E18, 9000, 1001).is_err());
    assert!(calculate_bid_split(E18, u16::MAX, u16::MAX).is_err());
    assert!(calculate_bid_split(E18, 9000, 1000).is_ok());
}
