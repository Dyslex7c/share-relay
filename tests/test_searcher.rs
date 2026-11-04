use mev_share_relay::bundle::searcher::SearcherRegistry;

#[test]
fn registered_searchers_are_authorized() {
    let registry = SearcherRegistry::default();
    let registered: Vec<[u8; 20]> = (0..10).map(|i| [i as u8; 20]).collect();
    for searcher in &registered {
        assert!(
            !registry.is_authorized(searcher),
            "pre-registration must deny"
        );
        registry.register_searcher(*searcher);
    }
    for searcher in &registered {
        assert!(
            registry.is_authorized(searcher),
            "registered searcher must be authorized"
        );
    }
    assert!(!registry.is_authorized(&[0xff; 20]));
}
