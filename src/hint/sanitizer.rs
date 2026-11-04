use serde::{Deserialize, Serialize};

use super::tx::PrivateTransaction;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SanitizedEvent {
    pub hash: Option<[u8; 32]>,
    pub logs: Option<Vec<[u8; 32]>>,
    pub to: Option<[u8; 20]>,
    pub function_selector: Option<[u8; 4]>,
    pub call_data: Option<Vec<u8>>,
}

pub fn sanitize_transaction(tx: &PrivateTransaction, logs: &[[u8; 32]]) -> SanitizedEvent {
    let h = &tx.hints;
    let selector = tx.data.get(..4).and_then(|b| <[u8; 4]>::try_from(b).ok());
    SanitizedEvent {
        hash: h.share_hash.then_some(tx.tx_hash),
        logs: h.share_logs.then(|| logs.to_vec()),
        to: h.share_contract_address.then_some(tx.to).flatten(),
        function_selector: h.share_function_selector.then_some(selector).flatten(),
        call_data: h.share_calldata.then(|| tx.data.clone()),
    }
}
