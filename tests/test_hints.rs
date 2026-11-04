#[path = "../src/hint/types.rs"]
mod types;

use types::HintPreferences;

#[test]
fn default_is_hash_only() {
    let p = HintPreferences::default();
    assert!(p.share_hash && !p.share_logs && !p.share_contract_address
        && !p.share_function_selector && !p.share_calldata);
}

#[test]
fn json_roundtrip_custom_hints() {
    for mask in 0_u8..32 {
        let p = HintPreferences {
            share_hash: mask & 1 != 0, share_logs: mask & 2 != 0,
            share_contract_address: mask & 4 != 0,
            share_function_selector: mask & 8 != 0, share_calldata: mask & 16 != 0,
        };
        let json = serde_json::to_string(&p).expect("serialize");
        assert_eq!(serde_json::from_str::<HintPreferences>(&json).expect("deserialize"), p);
    }
}

#[test]
fn privacy_preserving_json_schema() {
    let value = serde_json::to_value(HintPreferences::default()).expect("to_value");
    assert_eq!(value, serde_json::json!({
        "share_hash": true, "share_logs": false, "share_contract_address": false,
        "share_function_selector": false, "share_calldata": false
    }));
}