use std::cmp::Ordering;

use super::result::SimulationResult;

pub fn calculate_effective_gas_price(sim: &SimulationResult) -> u128 {
    sim.bid_amount / (sim.gas_used.max(1) as u128)
}

pub fn compare_bundles(a: &SimulationResult, b: &SimulationResult) -> Ordering {
    calculate_effective_gas_price(b)
        .cmp(&calculate_effective_gas_price(a))
        .then_with(|| b.user_refund_amount.cmp(&a.user_refund_amount))
}
