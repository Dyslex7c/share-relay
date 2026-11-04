use crate::bundle::{item::BundleItem, types::SearcherBundle};

#[derive(Clone, Debug, Default)]
pub struct BuilderDispatcher {
    pub builder_urls: Vec<String>,
}

impl BuilderDispatcher {
    pub fn new(builder_urls: Vec<String>) -> Self {
        Self { builder_urls }
    }
}

pub fn format_mev_share_bundle_payload(
    target_tx: &[u8],
    searcher_bundle: &SearcherBundle,
) -> Vec<u8> {
    let mut payload = target_tx.to_vec();
    for item in &searcher_bundle.items {
        if let BundleItem::SignedTransaction { raw } = item {
            payload.extend_from_slice(raw);
        }
    }
    payload
}
