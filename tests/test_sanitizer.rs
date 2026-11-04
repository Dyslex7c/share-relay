use mev_share_relay::hint::sanitizer::sanitize_transaction;
use mev_share_relay::hint::tx::PrivateTransaction;
use mev_share_relay::hint::types::HintPreferences;

fn private_tx(hints: HintPreferences) -> PrivateTransaction {
    PrivateTransaction::new(
        [1; 32],
        [2; 20],
        Some([3; 20]),
        100_000,
        10,
        vec![0xa9, 0x05, 0x9c, 0xbb, 0xff],
        hints,
    )
    .expect("gas must be nonzero")
}

#[test]
fn hash_only_default_masks_sensitive_fields() {
    let event = sanitize_transaction(&private_tx(HintPreferences::default()), &[]);
    assert_eq!(event.hash, Some([1; 32]));
    assert!(event.to.is_none());
    assert!(event.function_selector.is_none());
    assert!(event.call_data.is_none());
}

#[test]
fn contract_hint_only_reveals_to() {
    let event = sanitize_transaction(
        &private_tx(HintPreferences {
            share_contract_address: true,
            ..Default::default()
        }),
        &[],
    );
    assert_eq!(event.to, Some([3; 20]));
    assert!(event.call_data.is_none());
}
