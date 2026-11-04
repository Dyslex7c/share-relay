use mev_share_relay::bundle::{item::BundleItem, types::SearcherBundle};
use mev_share_relay::hint::tx::PrivateTransaction;
use mev_share_relay::matchmaker::{merger::merge_winning_bundles, result::SimulationResult};
use mev_share_relay::matchmaker::sim::simulate_bundle_dry_run;
use mev_share_relay::orchestrator::MatchmakerEngine;
#[test]
fn higher_bid_wins_and_pays_user_and_builder() {
    let engine = MatchmakerEngine::new();
    let mut subscriber = engine.broadcaster.subscribe();
    let tx = PrivateTransaction::new([9; 32], [1; 20], Some([2; 20]), 100_000, 1, vec![3; 4], Default::default()).expect("valid transaction");
    let user = tx.from;
    let hash = tx.tx_hash;
    let event = engine.process_incoming_tx(tx).expect("subscribed searcher");
    assert_eq!(subscriber.try_recv().expect("published hint"), event);
    assert_eq!(event.hash, Some(hash));
    let mut candidates = [(1_u8, 500_000_000_000_000_000_u128), (2, 800_000_000_000_000_000)].map(|(id, bid)| {
        let bundle = SearcherBundle {
            bundle_id: [id; 32],
            items: vec![BundleItem::TxHashReference { hash }, BundleItem::SignedTransaction { raw: vec![id] }],
            target_block: 1_000, min_timestamp: None, max_timestamp: None, kickback_bps: 9000,
        };
        let outcome = simulate_bundle_dry_run(&bundle, bid);
        SimulationResult {
            bundle_id: bundle.bundle_id, target_hash: hash, gas_used: outcome.gas_used,
            bid_amount: bid, user_refund_amount: u128::try_from(outcome.user_balance_delta).expect("positive refund"),
            builder_payment: outcome.builder_payment,
        }
    });
    let winners = merge_winning_bundles(&mut candidates);
    assert!(winners.len() == 1 && winners[0].bundle_id == [2; 32]);
    assert_eq!(winners[0].builder_payment, 80_000_000_000_000_000);
    engine.kickbacks.credit_refund(user, winners[0].user_refund_amount);
    assert_eq!(engine.kickbacks.claim_refund(&user), 720_000_000_000_000_000);
}
