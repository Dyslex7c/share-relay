#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationResult {
    pub bundle_id: [u8; 32],
    pub target_hash: [u8; 32],
    pub gas_used: u64,
    pub bid_amount: u128,
    pub user_refund_amount: u128,
    pub builder_payment: u128,
}
