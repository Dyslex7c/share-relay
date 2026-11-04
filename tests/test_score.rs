use mev_share_relay::matchmaker::result::SimulationResult;
use mev_share_relay::matchmaker::score::{calculate_effective_gas_price, compare_bundles};
use std::cmp::Ordering;

fn sim(id: u8, gas_used: u64, bid_amount: u128, user_refund_amount: u128) -> SimulationResult {
    SimulationResult {
        bundle_id: [id; 32],
        target_hash: [0; 32],
        gas_used,
        bid_amount,
        user_refund_amount,
        builder_payment: 0,
    }
}

#[test]
fn lower_gas_bundle_wins_on_effective_gas_price() {
    let a = sim(1, 100_000, 2_000_000_000_000_000_000, 0);
    let b = sim(2, 50_000, 1_500_000_000_000_000_000, 0);
    assert_eq!(calculate_effective_gas_price(&a), 20_000_000_000_000);
    assert_eq!(calculate_effective_gas_price(&b), 30_000_000_000_000);
    assert_eq!(compare_bundles(&a, &b), Ordering::Greater);
}

#[test]
fn equal_gas_price_ties_break_on_user_refund() {
    let low = sim(1, 50_000, 1_500_000_000_000_000_000, 1);
    let high = sim(2, 50_000, 1_500_000_000_000_000_000, 2);
    assert_eq!(
        calculate_effective_gas_price(&low),
        calculate_effective_gas_price(&high)
    );
    assert_eq!(compare_bundles(&low, &high), Ordering::Greater);
}
