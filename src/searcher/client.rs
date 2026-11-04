use crate::bundle::types::SearcherBundle;
use serde_json::json;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearcherRpcClient {
    pub relay_url: String,
}

impl SearcherRpcClient {
    pub fn new(relay_url: String) -> Self {
        Self { relay_url }
    }

    pub fn format_send_bundle_request(&self, bundle: &SearcherBundle) -> String {
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "mev_sendBundle",
            "params": [bundle],
        })
        .to_string()
    }
}
