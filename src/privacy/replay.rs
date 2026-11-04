use parking_lot::RwLock;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Default)]
pub struct ReplayFilter {
    seen_bundles: RwLock<HashMap<[u8; 32], u64>>,
}

impl ReplayFilter {
    pub fn check_and_mark(
        &self,
        bundle_id: [u8; 32],
        target_block: u64,
    ) -> Result<(), &'static str> {
        match self.seen_bundles.write().entry(bundle_id) {
            Entry::Occupied(_) => Err("bundle id already seen"),
            Entry::Vacant(slot) => {
                slot.insert(target_block);
                Ok(())
            }
        }
    }

    pub fn prune_older_than(&self, current_block: u64) {
        self.seen_bundles
            .write()
            .retain(|_, target_block| *target_block >= current_block);
    }
}
