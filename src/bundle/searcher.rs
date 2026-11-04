use parking_lot::RwLock;
use std::collections::HashSet;

#[derive(Default)]
pub struct SearcherRegistry {
    approved_searchers: RwLock<HashSet<[u8; 20]>>,
}

impl SearcherRegistry {
    pub fn register_searcher(&self, searcher: [u8; 20]) {
        self.approved_searchers.write().insert(searcher);
    }

    pub fn is_authorized(&self, searcher: &[u8; 20]) -> bool {
        self.approved_searchers.read().contains(searcher)
    }
}

#[cfg(test)]
mod tests {
    use super::SearcherRegistry;
    #[test]
    fn registration_is_shared_across_threads() {
        let registry = SearcherRegistry::default();
        assert!(!registry.is_authorized(&[1; 20]));
        std::thread::scope(|scope| {
            scope.spawn(|| registry.register_searcher([1; 20]));
        });
        registry.register_searcher([1; 20]);
        assert!(registry.is_authorized(&[1; 20]));
        assert!(!registry.is_authorized(&[2; 20]));
    }
}
