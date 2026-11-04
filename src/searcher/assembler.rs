// Sources: [S30] this crate, `src/bundle/types.rs:5-13` + `src/bundle/item.rs:3-7` — verified.
// `[S12]` `src/bundle/validator.rs:7-22` requires a hash reference before the signed tx.
// `bundle_id` is caller-assigned elsewhere in this crate (`unknown` derivation), so it is
// zeroed here and must be set by the submitter; `min/max_timestamp` stay `None` (unbounded).
use crate::bundle::item::BundleItem;
use crate::bundle::types::SearcherBundle;

pub fn assemble_backrun_bundle(
    target_hash: [u8; 32],
    searcher_raw_tx: Vec<u8>,
    target_block: u64,
    kickback_bps: u16,
) -> SearcherBundle {
    let items = vec![
        BundleItem::TxHashReference { hash: target_hash },
        BundleItem::SignedTransaction {
            raw: searcher_raw_tx,
        },
    ];
    SearcherBundle {
        bundle_id: [0u8; 32],
        items,
        target_block,
        min_timestamp: None,
        max_timestamp: None,
        kickback_bps,
    }
}
