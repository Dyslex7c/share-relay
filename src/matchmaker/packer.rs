use crate::bundle::types::SearcherBundle;

#[derive(Clone, Debug)]
pub struct MegaBundle {
    pub block_number: u64,
    pub winning_bundles: Vec<SearcherBundle>,
    pub total_builder_payment: u128,
    pub total_user_refunds: u128,
}

pub fn pack_mega_bundle(
    block: u64,
    merged: Vec<SearcherBundle>,
    payments: &[(u128, u128)],
) -> MegaBundle {
    let (total_builder_payment, total_user_refunds) = payments.iter().fold(
        (0_u128, 0_u128),
        |(builder_total, user_total), &(builder, refund)| {
            (
                builder_total.saturating_add(builder),
                user_total.saturating_add(refund),
            )
        },
    );
    MegaBundle {
        block_number: block,
        winning_bundles: merged,
        total_builder_payment,
        total_user_refunds,
    }
}
