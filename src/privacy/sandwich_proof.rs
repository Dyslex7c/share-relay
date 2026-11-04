use crate::bundle::item::BundleItem;

pub fn verify_strict_backrun_ordering(
    target_hash: &[u8; 32],
    bundle_items: &[BundleItem],
) -> Result<(), &'static str> {
    match bundle_items.first() {
        Some(BundleItem::TxHashReference { hash }) if hash == target_hash => {}
        Some(BundleItem::TxHashReference { .. }) => {
            return Err("first item must reference the target transaction");
        }
        Some(BundleItem::SignedTransaction { .. }) => {
            return Err("transaction placed before the target transaction");
        }
        None => return Err("bundle is empty"),
    }
    if bundle_items.len() < 2
        || !bundle_items[1..]
            .iter()
            .all(|item| matches!(item, BundleItem::SignedTransaction { .. }))
    {
        return Err("only signed backrun transactions may follow the target");
    }
    Ok(())
}
