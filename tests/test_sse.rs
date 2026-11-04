use mev_share_relay::hint::sanitizer::SanitizedEvent;
use mev_share_relay::hint::sse::SseBroadcaster;

#[test]
fn broadcast_fans_out_event_to_all_searchers() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime");
    runtime.block_on(async {
        let broadcaster = SseBroadcaster::new(16);
        let mut searcher_one = broadcaster.subscribe();
        let mut searcher_two = broadcaster.subscribe();
        let event = SanitizedEvent {
            hash: Some([9; 32]),
            ..Default::default()
        };
        assert_eq!(broadcaster.publish(event.clone()), Ok(2));
        let received_one = searcher_one.recv().await.expect("searcher one");
        let received_two = searcher_two.recv().await.expect("searcher two");
        assert_eq!(received_one, event);
        assert_eq!(received_two, event);
        assert_eq!(received_one, received_two);
    });
}
