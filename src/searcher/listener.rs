use crate::hint::sanitizer::SanitizedEvent;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct HintFilter {
    pub monitored_contracts: HashSet<[u8; 20]>,
    pub monitored_selectors: HashSet<[u8; 4]>,
}

pub fn should_process_event(filter: &HintFilter, event: &SanitizedEvent) -> bool {
    let wildcard = filter.monitored_contracts.is_empty() && filter.monitored_selectors.is_empty();
    wildcard
        || event
            .to
            .is_some_and(|address| filter.monitored_contracts.contains(&address))
        || event
            .function_selector
            .is_some_and(|selector| filter.monitored_selectors.contains(&selector))
}
