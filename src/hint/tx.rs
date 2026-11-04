use serde::{Deserialize, Serialize};

use super::types;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateTransaction {
    pub tx_hash: [u8; 32],
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub gas_limit: u64,
    pub max_fee_per_gas: u128,
    pub data: Vec<u8>,
    pub hints: types::HintPreferences,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZeroGasLimit;
impl PrivateTransaction {
    pub fn new(
        tx_hash: [u8; 32],
        from: [u8; 20],
        to: Option<[u8; 20]>,
        gas_limit: u64,
        max_fee_per_gas: u128,
        data: Vec<u8>,
        hints: types::HintPreferences,
    ) -> Result<Self, ZeroGasLimit> {
        if gas_limit == 0 {
            return Err(ZeroGasLimit);
        }
        Ok(Self {
            tx_hash,
            from,
            to,
            gas_limit,
            max_fee_per_gas,
            data,
            hints,
        })
    }
}
