use mev_share_relay::searcher::pool::{AmmPool, UNISWAP_V2_FEE_BPS};

const UNIT: u128 = 1_000_000;
const TOKEN0: [u8; 20] = [0xa0; 20];
const TOKEN1: [u8; 20] = [0xb1; 20];

fn pool() -> AmmPool {
    AmmPool {
        pool_address: [0xcc; 20],
        token0: TOKEN0,
        token1: TOKEN1,
        reserve0: 1_000 * UNIT,
        reserve1: 2_000 * UNIT,
        fee_bps: UNISWAP_V2_FEE_BPS,
    }
}

#[test]
fn swap_output_matches_constant_product_within_one_unit() {
    let amount_in = 10 * UNIT;
    let p = pool();
    let out = p.get_amount_out(amount_in, &TOKEN0).expect("fits u128");
    let (r0, r1) = ((1_000 * UNIT) as f64, (2_000 * UNIT) as f64);
    let manual = r1 - (r0 * r1) / (r0 + amount_in as f64 * 0.997);
    let diff = (out as f64 - manual).abs();
    assert!(diff <= 1.0, "out={out} manual={manual}");
    assert!(out < 20 * UNIT, "fee and impact keep it below 2:1 spot");
}

#[test]
fn zero_input_and_foreign_token_are_rejected() {
    assert!(pool().get_amount_out(0, &TOKEN0).is_err());
    assert!(pool().get_amount_out(10 * UNIT, &[0xdd; 20]).is_err());
}
