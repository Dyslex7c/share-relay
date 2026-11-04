use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HintPreferences {
    pub share_hash: bool,
    pub share_logs: bool,
    pub share_contract_address: bool,
    pub share_function_selector: bool,
    pub share_calldata: bool,
}

impl Default for HintPreferences {
    fn default() -> Self {
        Self {
            share_hash: true,
            share_logs: false,
            share_contract_address: false,
            share_function_selector: false,
            share_calldata: false,
        }
    }
}
