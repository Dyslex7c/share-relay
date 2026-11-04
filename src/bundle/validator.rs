use super::{item::BundleItem, types::SearcherBundle};

pub fn validate_bundle_structure(
    bundle: &SearcherBundle,
    min_kickback_bps: u16,
) -> Result<(), &'static str> {
    let has_reference_then_signed = bundle
        .items
        .iter()
        .position(|item| matches!(item, BundleItem::SignedTransaction { .. }))
        .is_some_and(|index| {
            index > 0
                && bundle.items[..index]
                    .iter()
                    .all(|item| matches!(item, BundleItem::TxHashReference { .. }))
        });
    if !has_reference_then_signed {
        return Err("bundle requires a transaction reference followed by a signed transaction");
    }
    if bundle.kickback_bps < min_kickback_bps {
        return Err("bundle kickback is below the minimum");
    }
    if matches!(
        (bundle.min_timestamp, bundle.max_timestamp),
        (Some(min), Some(max)) if max < min
    ) {
        return Err("bundle timestamp range is invalid");
    }
    Ok(())
}
