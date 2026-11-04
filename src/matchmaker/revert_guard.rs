use super::sim::ExecutionOutcome;
use crate::bundle::types::SearcherBundle;

pub fn filter_failing_bundles(
    outcomes: Vec<(SearcherBundle, ExecutionOutcome)>,
) -> Vec<SearcherBundle> {
    let mut outcomes = outcomes;
    outcomes
        .drain(..)
        .filter(|(_, outcome)| outcome.success && outcome.user_balance_delta >= 0)
        .map(|(bundle, _)| bundle)
        .collect()
}
