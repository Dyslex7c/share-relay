use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Default)]
pub struct KickbackLedger {
    pending_refunds: RwLock<HashMap<[u8; 20], u128>>,
}

impl KickbackLedger {
    pub fn credit_refund(&self, recipient: [u8; 20], amount: u128) {
        let mut pending = self.pending_refunds.write();
        let balance = pending.entry(recipient).or_default();
        *balance = balance.saturating_add(amount);
    }

    pub fn claim_refund(&self, recipient: &[u8; 20]) -> u128 {
        self.pending_refunds.write().remove(recipient).unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::KickbackLedger;
    #[test]
    fn credits_accumulate_and_claims_reset() {
        let ledger = KickbackLedger::default();
        ledger.credit_refund([1; 20], 40);
        ledger.credit_refund([1; 20], 2);
        ledger.credit_refund([2; 20], 7);
        assert_eq!(ledger.claim_refund(&[1; 20]), 42);
        assert_eq!(ledger.claim_refund(&[1; 20]), 0);
        assert_eq!(ledger.claim_refund(&[2; 20]), 7);
    }
}
