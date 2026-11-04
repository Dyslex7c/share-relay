use mev_share_relay::privacy::encrypted::EncryptedBundle;

fn mock_bundle() -> EncryptedBundle {
    EncryptedBundle {
        bundle_id: [1; 32],
        builder_pubkey: [2; 32],
        ephemeral_pubkey: [3; 32],
        ciphertext: (0..64).map(|i| i as u8).collect(),
        nonce: [4; 12],
        tag: [5; 16],
    }
}

#[test]
fn json_roundtrip_preserves_every_byte() {
    let original = mock_bundle();
    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: EncryptedBundle = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(decoded, original);
    assert_eq!(decoded.bundle_id, [1; 32]);
    assert_eq!(decoded.builder_pubkey, [2; 32]);
    assert_eq!(decoded.ephemeral_pubkey, [3; 32]);
    assert_eq!(
        decoded.ciphertext,
        (0..64).map(|i| i as u8).collect::<Vec<u8>>()
    );
    assert_eq!(decoded.ciphertext.len(), 64);
    assert_eq!(decoded.nonce, [4; 12]);
    assert_eq!(decoded.tag, [5; 16]);
}
