use crate::decimal::{Decimal, DivUp, Mul, Sqrt, Sub};
use wasm_bindgen::prelude::wasm_bindgen;

pub const MIN_LIQUIDITY: u64 = 100;
pub const LIQUIDITY_POOL_SCALE: u8 = 9;

#[wasm_bindgen]
pub fn calculate_k(x: u64, x_scale: u8, y: u64, y_scale: u8) -> Option<u64> {
    let x = Decimal::from_scaled_amount(x, x_scale).to_compute_scale();
    let y = Decimal::from_scaled_amount(y, y_scale).to_compute_scale();
    let min_liquidity =
        Decimal::from_scaled_amount(MIN_LIQUIDITY, LIQUIDITY_POOL_SCALE).to_compute_scale();

    // sqrt(x * y) - min_liquidity
    Some(
        x.mul(y)
            .sqrt()
            .unwrap()
            .sub(min_liquidity)
            .unwrap()
            .to_scaled_amount(LIQUIDITY_POOL_SCALE),
    )
}

/// calculate x and y from k
pub fn calculate_x_y(
    lp_tokens: u64,
    lp_tokens_scale: u8,
    x: u64,
    x_scale: u8,
    y: u64,
    y_scale: u8,
    lp_tokens_total: u64,
) -> (u64, u64) {
    let x_total = Decimal::from_scaled_amount(x, x_scale);
    let y_total = Decimal::from_scaled_amount(y, y_scale);
    let lp_total = Decimal::from_scaled_amount(lp_tokens_total, lp_tokens_scale);
    let lp_tokens_to_mint = Decimal::from_scaled_amount(lp_tokens, lp_tokens_scale);

    // div up (ceiling) as we are receiving these amounts
    let x_debited = lp_tokens_to_mint
        .mul(x_total)
        .div_up(lp_total)
        .to_scaled_amount(x_scale);
    let y_debited = lp_tokens_to_mint
        .mul(y_total)
        .div_up(lp_total)
        .to_scaled_amount(y_scale);

    (x_debited, y_debited)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculate_k() {
        {
            // Same scale
            // (600.000000 * 20.000000)**0.5 - 100/10^9 = 109.54451140103322269139
            let expected: u64 = 109_544511401;
            let result = calculate_k(600_000000, 6, 20_000000, 6).unwrap();
            assert_eq!(expected, result);
        }

        {
            // Different scale
            // (600.000000 * 20.000000000)**0.5 - 100/10^9 = 9.544511
            let expected: u64 = 109_544511401;
            let result = calculate_k(600_000000, 6, 20_000000000, 9).unwrap();
            assert_eq!(expected, result);
        }

        {
            // Different scale
            // (600.000000000 * 20.000000)**0.5 - 100/10^9 = 9.544511501
            let expected: u64 = 109_544511401;
            let result = calculate_k(600_000000, 9, 20_000000000, 6).unwrap();
            assert_eq!(expected, result);
        }
    }
}
