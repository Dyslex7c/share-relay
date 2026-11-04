// Sources: [S19] Uniswap v2-periphery `UniswapV2Library.getAmountOut` (997/1000 scaling) — verified.
// [S20] Uniswap v2-core `UniswapV2Pair.swap` K invariant (`balance*1000 - amountIn*3`) — verified.
pub const UNISWAP_V2_FEE_BPS: u16 = 30;
pub type AmountOut = Result<u128, &'static str>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmmPool {
    pub pool_address: [u8; 20],
    pub token0: [u8; 20],
    pub token1: [u8; 20],
    pub reserve0: u128,
    pub reserve1: u128,
    pub fee_bps: u16,
}
impl AmmPool {
    pub fn get_amount_out(&self, amount_in: u128, token_in: &[u8; 20]) -> AmountOut {
        if self.fee_bps != UNISWAP_V2_FEE_BPS {
            return Err("only the 30 bps Uniswap v2 fee is modelled");
        }
        let (reserve_in, reserve_out) = match *token_in {
            t if t == self.token0 => (self.reserve0, self.reserve1),
            t if t == self.token1 => (self.reserve1, self.reserve0),
            _ => return Err("token is not part of this pool"),
        };
        if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
            return Err("insufficient input amount or liquidity");
        }
        let with_fee = amount_in.checked_mul(997).ok_or("amount_in overflow")?;
        let numerator = with_fee.checked_mul(reserve_out).ok_or("output overflow")?;
        let scaled_in = reserve_in.checked_mul(1000).ok_or("reserve overflow")?;
        let denominator = scaled_in.checked_add(with_fee).ok_or("divisor overflow")?;
        Ok(numerator / denominator)
    }
}
