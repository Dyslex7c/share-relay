// Sources: [M1] flashbots/mev-share `specs/` — status: to-verify; the 90% refund default is
// `unknown` (`docs/kickback-economics.md`), so `kickback_bps` is caller-supplied, not assumed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BidSplit {
    pub searcher_profit: u128,
    pub user_kickback: u128,
    pub builder_payment: u128,
}
const BPS_TOTAL: u32 = 10_000;
fn bps_of(amount: u128, bps: u16) -> Result<u128, &'static str> {
    let numerator = u128::from(bps)
        .checked_mul(amount)
        .ok_or("bps numerator overflow")?;
    Ok(numerator / u128::from(BPS_TOTAL))
}
pub fn calculate_bid_split(
    net_profit: u128,
    kickback_bps: u16,
    builder_share_bps: u16,
) -> Result<BidSplit, &'static str> {
    let combined = u32::from(kickback_bps) + u32::from(builder_share_bps);
    if combined > BPS_TOTAL {
        return Err("kickback_bps + builder_share_bps exceed 10000 bps");
    }
    let user_kickback = bps_of(net_profit, kickback_bps)?;
    let remainder = net_profit - user_kickback;
    let builder_payment = bps_of(net_profit, builder_share_bps)?;
    Ok(BidSplit {
        searcher_profit: remainder - builder_payment,
        user_kickback,
        builder_payment,
    })
}
