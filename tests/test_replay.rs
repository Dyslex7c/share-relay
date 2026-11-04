use mev_share_relay::privacy::replay::ReplayFilter;

#[test]
fn replay_is_rejected_until_target_block_is_pruned() {
    let filter = ReplayFilter::default();
    let bundle_one = [1; 32];

    assert_eq!(filter.check_and_mark(bundle_one, 100), Ok(()));
    assert!(filter.check_and_mark(bundle_one, 100).is_err());
    assert!(filter.check_and_mark(bundle_one, 102).is_err());

    filter.prune_older_than(101);
    assert_eq!(filter.check_and_mark(bundle_one, 102), Ok(()));
}
