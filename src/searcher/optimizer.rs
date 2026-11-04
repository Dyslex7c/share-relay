// Sources: [S19] Uniswap v2-periphery `UniswapV2Library.getAmountsOut` (hop chaining) — verified.
use super::pool::AmmPool;
const STEPS: u8 = 20;
fn net(a: &AmmPool, b: &AmmPool, token: &[u8; 20], amount: u128) -> Option<i128> {
    let mid = [a.token0, a.token1][usize::from(*token == a.token0)];
    let first = a.get_amount_out(amount, token).ok()?;
    let second = b.get_amount_out(first, &mid).ok()?;
    let (s, x) = (i128::try_from(second).ok()?, i128::try_from(amount).ok()?);
    Some(s.saturating_sub(x))
}
pub fn optimize_arb_input(
    a: &AmmPool,
    b: &AmmPool,
    token: &[u8; 20],
    max_borrow: u128,
) -> (u128, u128) {
    if max_borrow == 0 {
        return (0, 0);
    }
    let (mut lo, mut hi) = (1u128, max_borrow);
    let mut best = (0u128, 0i128);
    for _ in 0..STEPS {
        let third = hi.saturating_sub(lo) / 3;
        let (m1, m2) = (lo + third, hi.saturating_sub(third));
        let f = |x: u128| net(a, b, token, x).unwrap_or(0);
        let (f1, f2) = (f(m1), f(m2));
        let (mp, ma) = if f1 > f2 { (f1, m1) } else { (f2, m2) };
        best = if mp > best.1 { (ma, mp) } else { best };
        let (new_lo, new_hi) = (m1.saturating_add(1), m2.saturating_sub(1));
        lo = if f1 < f2 { new_lo } else { lo };
        hi = if f1 < f2 { hi } else { new_hi };
    }
    (best.0, u128::try_from(best.1).unwrap_or(0))
}
