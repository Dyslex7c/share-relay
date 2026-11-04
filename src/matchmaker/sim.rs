use crate::bundle::types::SearcherBundle;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionOutcome {
    pub success: bool,
    pub gas_used: u64,
    pub logs_emitted: Vec<[u8; 32]>,
    pub user_balance_delta: i128,
    pub builder_payment: u128,
}

pub fn simulate_bundle_dry_run(bundle: &SearcherBundle, target_value: u128) -> ExecutionOutcome {
    let bps = u128::from(bundle.kickback_bps);
    let refund = if bps >= 10_000 {
        target_value
    } else {
        target_value / 10_000 * bps + target_value % 10_000 * bps / 10_000
    };
    let user_balance_delta = i128::try_from(refund).unwrap_or(i128::MAX);
    ExecutionOutcome {
        success: true,
        gas_used: 0,
        logs_emitted: Vec::new(),
        user_balance_delta,
        builder_payment: target_value - refund,
    }
}
