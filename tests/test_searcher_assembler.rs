use mev_share_relay::bundle::item::BundleItem;
use mev_share_relay::searcher::assembler::assemble_backrun_bundle;

const TARGET: [u8; 32] = [0xab; 32];
const RAW_TX: [u8; 4] = [0x02, 0xf8, 0x6c, 0x01];

#[test]
fn assembles_reference_then_signed_backrun_items() {
    let bundle = assemble_backrun_bundle(TARGET, RAW_TX.to_vec(), 1_000, 9000);
    assert_eq!(bundle.items.len(), 2);
    assert!(matches!(
        &bundle.items[0],
        BundleItem::TxHashReference { .. }
    ));
    assert!(matches!(
        &bundle.items[1],
        BundleItem::SignedTransaction { .. }
    ));
    assert_eq!(bundle.target_block, 1_000);
    assert_eq!(bundle.kickback_bps, 9000);
}

#[test]
fn target_hash_and_searcher_bytes_are_preserved_verbatim() {
    let bundle = assemble_backrun_bundle(TARGET, RAW_TX.to_vec(), 1_000, 9000);
    let BundleItem::TxHashReference { hash } = &bundle.items[0] else {
        panic!("index 0 must be a tx hash reference");
    };
    let BundleItem::SignedTransaction { raw } = &bundle.items[1] else {
        panic!("index 1 must be a signed transaction");
    };
    assert_eq!(hash, &TARGET);
    assert_eq!(raw.as_slice(), RAW_TX);
}
