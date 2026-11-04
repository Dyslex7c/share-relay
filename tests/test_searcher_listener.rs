use mev_share_relay::hint::sanitizer::SanitizedEvent as Event;
use mev_share_relay::searcher::listener::{should_process_event, HintFilter};

fn configured_filter() -> HintFilter {
    HintFilter {
        monitored_contracts: [[0x11; 20]].into(),
        monitored_selectors: [[0x38, 0xed, 0x17, 0x39]].into(),
    }
}
#[test]
fn matching_event_is_processed() {
    let event = Event {
        to: Some([0x11; 20]),
        function_selector: Some([0x38, 0xed, 0x17, 0x39]),
        ..Default::default()
    };
    assert!(should_process_event(&configured_filter(), &event));
}

#[test]
fn unrelated_event_is_ignored() {
    let event = Event {
        to: Some([0x22; 20]),
        function_selector: Some([0xaa; 4]),
        ..Default::default()
    };
    assert!(!should_process_event(&configured_filter(), &event));
}

#[test]
fn empty_filter_accepts_even_fully_masked_events() {
    let filter = HintFilter::default();
    assert!(should_process_event(&filter, &Event::default()));
}
