use mev_share_relay::matchmaker::kickback::KickbackLedger;

#[test]
fn refunds_accumulate_and_can_only_be_claimed_once() {
    let ledger = KickbackLedger::default();
    let user_a = [1_u8; 20];

    ledger.credit_refund(user_a, 500_000_000_000_000_000);
    ledger.credit_refund(user_a, 300_000_000_000_000_000);

    assert_eq!(ledger.claim_refund(&user_a), 800_000_000_000_000_000);
    assert_eq!(ledger.claim_refund(&user_a), 0);
}
