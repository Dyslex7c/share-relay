use mev_share_relay::searcher::optimizer::optimize_arb_input;
use mev_share_relay::searcher::pool::{AmmPool, UNISWAP_V2_FEE_BPS};

const ETH: [u8; 20] = [0xe7; 20];
const USDC: [u8; 20] = [0xdc; 20];
const E18: u128 = 1_000_000_000_000_000_000;
const E6: u128 = 1_000_000;
const MAX_BORROW: u128 = 20 * E18;
fn pool(addr: u8, eth: u128, usdc: u128) -> AmmPool {
    AmmPool {
        pool_address: [addr; 20],
        token0: ETH,
        token1: USDC,
        reserve0: eth,
        reserve1: usdc,
        fee_bps: UNISWAP_V2_FEE_BPS,
    }
}
fn net_profit(sell: &AmmPool, buy: &AmmPool, amount: u128) -> i128 {
    let usdc = sell.get_amount_out(amount, &ETH).expect("sell hop");
    let back = buy.get_amount_out(usdc, &USDC).expect("buy hop");
    i128::try_from(back).expect("fits") - i128::try_from(amount).expect("fits")
}

#[test]
fn optimum_is_interior_and_locally_maximal() {
    let a = pool(0xaa, 100 * E18, 200_000 * E6);
    let b = pool(0xbb, 100 * E18, 210_000 * E6);
    let (amount, profit) = optimize_arb_input(&b, &a, &ETH, MAX_BORROW);
    assert!(amount > 0, "amount {amount}");
    assert!(amount < MAX_BORROW, "amount {amount}");
    assert_eq!(
        net_profit(&b, &a, amount),
        i128::try_from(profit).expect("fits")
    );
    for delta in [E18 / 100, E18 / 10, E18] {
        assert!(net_profit(&b, &a, amount - delta) < i128::try_from(profit).expect("fits"));
        assert!(net_profit(&b, &a, amount + delta) < i128::try_from(profit).expect("fits"));
    }
}

#[test]
fn wrong_direction_and_zero_budget_return_zero() {
    let a = pool(0xaa, 100 * E18, 200_000 * E6);
    let b = pool(0xbb, 100 * E18, 210_000 * E6);
    assert_eq!(optimize_arb_input(&a, &b, &ETH, MAX_BORROW), (0, 0));
    assert_eq!(optimize_arb_input(&b, &a, &ETH, 0), (0, 0));
}
