use mev_share_relay::hint::tx::PrivateTransaction;
use mev_share_relay::hint::types::HintPreferences;
use mev_share_relay::orchestrator::MatchmakerEngine;

#[test]
fn subscriber_receives_hash_only_event_for_default_hints() {
    let engine = MatchmakerEngine::new();
    let mut searcher = engine.broadcaster.subscribe();
    let swap = PrivateTransaction::new(
        [0xab; 32],
        [1; 20],
        Some([2; 20]),
        210_000,
        30_000_000_000,
        vec![0x38, 0xed, 0x17, 0x39, 0xff],
        HintPreferences::default(),
    )
    .expect("nonzero gas limit");

    let published = engine.process_incoming_tx(swap).expect("one subscriber");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime");
    let received = runtime
        .block_on(searcher.recv())
        .expect("subscriber receives event");
    assert_eq!(received, published);
    assert_eq!(received.hash, Some([0xab; 32]));
    assert!(received.to.is_none());
    assert!(received.function_selector.is_none());
    assert!(received.call_data.is_none());
}
