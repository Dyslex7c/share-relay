use mev_share_relay::bundle::{item::BundleItem, types::SearcherBundle};
use mev_share_relay::matchmaker::packer::pack_mega_bundle;

#[test]
fn packs_three_winners_and_sums_builder_and_user_payments() {
    let winners = (1_u8..=3)
        .map(|id| SearcherBundle {
            bundle_id: [id; 32],
            items: vec![
                BundleItem::TxHashReference { hash: [id; 32] },
                BundleItem::SignedTransaction { raw: vec![id] },
            ],
            target_block: 1_000,
            min_timestamp: None,
            max_timestamp: None,
            kickback_bps: 9000,
        })
        .collect();
    let payments = [(10, 90), (20, 180), (30, 270)];
    let packed = pack_mega_bundle(1_000, winners, &payments);

    assert_eq!(packed.block_number, 1_000);
    assert_eq!(packed.winning_bundles.len(), 3);
    assert_eq!(packed.total_builder_payment, 60);
    assert_eq!(packed.total_user_refunds, 540);
    for (bundle, id) in packed.winning_bundles.iter().zip(1_u8..=3) {
        assert_eq!(bundle.bundle_id, [id; 32]);
        assert_eq!(bundle.target_block, 1_000);
        assert_eq!(
            bundle.items[0],
            BundleItem::TxHashReference { hash: [id; 32] }
        );
    }
}
