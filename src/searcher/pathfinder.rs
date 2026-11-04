// Sources: [S12] this crate `src/searcher/pool.rs:15-32` (amount-out API reused) — verified.
// [S19] Uniswap v2-periphery `UniswapV2Library.getAmountsOut` (chained hops) — verified.
use super::pool::AmmPool;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArbitrageOpportunity {
    pub pool_a: [u8; 20],
    pub pool_b: [u8; 20],
    pub token_borrow: [u8; 20],
    pub expected_gross_profit: u128,
}
pub fn detect_2hop_arb(
    pool_a: &AmmPool,
    pool_b: &AmmPool,
    test_amount: u128,
    token: &[u8; 20],
) -> Option<ArbitrageOpportunity> {
    let mid = match *token {
        t if t == pool_a.token0 => pool_a.token1,
        t if t == pool_a.token1 => pool_a.token0,
        _ => return None,
    };
    let hop1 = pool_a.get_amount_out(test_amount, token).ok()?;
    let hop2 = pool_b.get_amount_out(hop1, &mid).ok()?;
    if hop2 <= test_amount {
        return None;
    }
    Some(ArbitrageOpportunity {
        pool_a: pool_a.pool_address,
        pool_b: pool_b.pool_address,
        token_borrow: *token,
        expected_gross_profit: hop2 - test_amount,
    })
}
