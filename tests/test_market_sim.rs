use mev_share_relay::sim::market::{run_synthetic_market_auction, MarketSimulation};

fn market() -> MarketSimulation {
    MarketSimulation {
        searcher_count: 20,
        private_tx_count: 5,
        kickback_threshold_bps: 9_000,
    }
}

#[test]
fn one_winner_per_private_tx_with_positive_payouts() {
    let (winners, builder_total, kickback_total) = run_synthetic_market_auction(&market());
    assert_eq!(winners, 5);
    assert!(kickback_total > 0, "kickbacks {kickback_total}");
    assert!(builder_total > 0, "builder payments {builder_total}");
    assert!(kickback_total > builder_total);
}

#[test]
fn auction_rounds_are_deterministic() {
    let first = run_synthetic_market_auction(&market());
    assert_eq!(run_synthetic_market_auction(&market()), first);
}
