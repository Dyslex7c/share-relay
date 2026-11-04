use mev_share_relay::bundle::item::BundleItem;
use mev_share_relay::hint::sanitizer::SanitizedEvent;
use mev_share_relay::searcher::client::SearcherRpcClient;
use mev_share_relay::searcher::listener::HintFilter;
use mev_share_relay::searcher::worker::{SearcherBot, DEFAULT_KICKBACK_BPS};

const TX_HASH: [u8; 32] = [0xab; 32];
fn bot() -> SearcherBot {
    let client = SearcherRpcClient::new("https://relay.mev-share.net".into());
    SearcherBot::new(HintFilter::default(), client)
}
#[test]
fn hinted_tx_hash_produces_backrun_bundle_referencing_it() {
    let event = SanitizedEvent {
        hash: Some(TX_HASH),
        ..SanitizedEvent::default()
    };
    let bundle = bot().handle_hint_event(&event, 1_000).expect("bundle");
    let Some(BundleItem::TxHashReference { hash }) = bundle.items.first() else {
        panic!("index 0 must reference the target tx");
    };
    assert_eq!(hash, &TX_HASH);
    assert_eq!((bundle.items.len(), bundle.target_block), (2, 1_000));
    assert_eq!(bundle.kickback_bps, DEFAULT_KICKBACK_BPS);
}
#[test]
fn event_without_tx_hash_is_ignored() {
    let event = SanitizedEvent {
        to: Some([0x11; 20]),
        function_selector: Some([0xa9, 0x05, 0x9c, 0xbb]),
        ..SanitizedEvent::default()
    };
    assert!(bot().handle_hint_event(&event, 1_000).is_none());
}
