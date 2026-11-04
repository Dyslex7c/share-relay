use mev_share_relay::searcher::pathfinder::detect_2hop_arb;
use mev_share_relay::searcher::pool::{AmmPool, UNISWAP_V2_FEE_BPS};

const ETH: [u8; 20] = [0xe7; 20];
const USDC: [u8; 20] = [0xdc; 20];
const E18: u128 = 1_000_000_000_000_000_000;
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

#[test]
fn detects_mispriced_pool_opportunity_with_one_eth() {
    let a = pool(0xaa, 10_000 * E18, 20_000_000_000_000);
    let b = pool(0xbb, 10_000 * E18, 20_500_000_000_000);
    let opp = detect_2hop_arb(&b, &a, E18, &ETH).expect("sell dear, buy cheap");
    assert_eq!(opp.token_borrow, ETH);
    assert_eq!(opp.expected_gross_profit, 18_653_878_484_387_752);
    assert!(opp.expected_gross_profit > 0);
}

#[test]
fn balanced_pools_return_none() {
    let a = pool(0xaa, 10_000 * E18, 20_000_000_000_000);
    let b = pool(0xbb, 10_000 * E18, 20_000_000_000_000);
    assert!(detect_2hop_arb(&b, &a, E18, &ETH).is_none());
    assert!(detect_2hop_arb(&a, &b, E18, &ETH).is_none());
}
