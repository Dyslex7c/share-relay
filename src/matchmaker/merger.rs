use std::collections::HashSet;

use super::{result::SimulationResult, score::compare_bundles};

pub fn merge_winning_bundles(candidates: &mut [SimulationResult]) -> Vec<SimulationResult> {
    candidates.sort_by(compare_bundles);
    let mut claimed = HashSet::new();
    candidates
        .iter()
        .filter(|candidate| claimed.insert(candidate.target_hash))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keeps_highest_ranked_per_target() {
        let s = |id, target, bid, refund| SimulationResult {
            bundle_id: [id; 32],
            target_hash: [target; 32],
            gas_used: 1,
            bid_amount: bid,
            user_refund_amount: refund,
            builder_payment: 0,
        };
        let mut candidates = [s(1, 1, 20, 1), s(3, 2, 5, 1), s(4, 1, 20, 2)];
        let winners = merge_winning_bundles(&mut candidates);
        assert_eq!(winners, vec![s(4, 1, 20, 2), s(3, 2, 5, 1)]);
        assert!(merge_winning_bundles(&mut []).is_empty());
    }
}
