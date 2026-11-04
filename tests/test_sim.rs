use mev_share_relay::bundle::types::SearcherBundle;
use mev_share_relay::matchmaker::sim::simulate_bundle_dry_run;

const ETH: u128 = 1_000_000_000_000_000_000;

fn bundle(kickback_bps: u16) -> SearcherBundle {
    SearcherBundle {
        bundle_id: [0; 32],
        items: Vec::new(),
        target_block: 1,
        min_timestamp: None,
        max_timestamp: None,
        kickback_bps,
    }
}

#[test]
fn ninety_percent_kickback_splits_one_eth() {
    let outcome = simulate_bundle_dry_run(&bundle(9000), ETH);
    assert!(outcome.success);
    assert_eq!(outcome.user_balance_delta, 900_000_000_000_000_000);
    assert_eq!(outcome.builder_payment, 100_000_000_000_000_000);
    assert_eq!(
        i128::try_from(outcome.builder_payment).expect("builder payment fits i128")
            + outcome.user_balance_delta,
        i128::try_from(ETH).expect("profit fits i128")
    );
    assert!(outcome.logs_emitted.is_empty());
    assert_eq!(outcome.gas_used, 0);
    assert_eq!(outcome, simulate_bundle_dry_run(&bundle(9000), ETH));
}
