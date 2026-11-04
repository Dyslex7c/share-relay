use serde::{Deserialize, Serialize};

use super::item::BundleItem;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearcherBundle {
    pub bundle_id: [u8; 32],
    pub items: Vec<BundleItem>,
    pub target_block: u64,
    pub min_timestamp: Option<u64>,
    pub max_timestamp: Option<u64>,
    pub kickback_bps: u16,
}
