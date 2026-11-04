use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BundleItem {
    TxHashReference { hash: [u8; 32] },
    SignedTransaction { raw: Vec<u8> },
}
