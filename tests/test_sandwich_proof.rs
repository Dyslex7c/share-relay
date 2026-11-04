use mev_share_relay::bundle::item::BundleItem;
use mev_share_relay::privacy::sandwich_proof::verify_strict_backrun_ordering;

fn reference() -> BundleItem {
    BundleItem::TxHashReference { hash: [9; 32] }
}
fn backrun() -> BundleItem {
    BundleItem::SignedTransaction { raw: vec![1] }
}

#[test]
fn user_first_then_backrun_passes() {
    assert!(verify_strict_backrun_ordering(&[9; 32], &[reference(), backrun()]).is_ok());
}

#[test]
fn frontrun_at_index_zero_is_rejected() {
    let result = verify_strict_backrun_ordering(&[9; 32], &[backrun(), reference()]);
    assert!(result.is_err());
}

#[test]
fn sandwich_before_and_after_user_is_rejected() {
    let result = verify_strict_backrun_ordering(&[9; 32], &[backrun(), reference(), backrun()]);
    assert!(result.is_err());
}
