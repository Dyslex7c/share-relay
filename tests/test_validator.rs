use mev_share_relay::bundle::item::BundleItem;
use mev_share_relay::bundle::types::SearcherBundle;
use mev_share_relay::bundle::validator::validate_bundle_structure;

fn bundle(items: Vec<BundleItem>, kickback_bps: u16, min: Option<u64>, max: Option<u64>) -> SearcherBundle {
    SearcherBundle { bundle_id: [1; 32], items, target_block: 10, min_timestamp: min, max_timestamp: max, kickback_bps }
}
fn backrun() -> Vec<BundleItem> {
    vec![BundleItem::TxHashReference { hash: [2; 32] }, BundleItem::SignedTransaction { raw: vec![0x02] }]
}

#[test]
fn valid_reference_then_backrun_passes() {
    assert_eq!(validate_bundle_structure(&bundle(backrun(), 9000, None, None), 8000), Ok(()));
}

#[test]
fn rejects_insufficient_kickback() {
    assert!(validate_bundle_structure(&bundle(backrun(), 5000, None, None), 8000).is_err());
}

#[test]
fn rejects_missing_backrun_or_inverted_timestamps() {
    let no_backrun = bundle(vec![BundleItem::TxHashReference { hash: [2; 32] }], 9000, None, None);
    assert!(validate_bundle_structure(&no_backrun, 0).is_err());
    let inverted = bundle(backrun(), 9000, Some(200), Some(100));
    assert!(validate_bundle_structure(&inverted, 0).is_err());
}
