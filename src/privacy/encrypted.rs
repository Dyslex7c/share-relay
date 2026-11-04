use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedBundle {
    pub bundle_id: [u8; 32],
    pub builder_pubkey: [u8; 32],
    pub ephemeral_pubkey: [u8; 32],
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub tag: [u8; 16],
}
