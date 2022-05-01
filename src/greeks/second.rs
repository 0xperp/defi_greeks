// Module containing functions for calculating second order greeks
use std::f64::consts::E;

use common::*;

/// Calculates the Gamma for an option
///
/// Gamma measures the rate of change in the delta with respect to the change in the underlying price.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn gamma(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    return gamma_d1(s0, t, q, sigma, d1);
}

pub fn gamma_d1(s0: f64, t: f64, q: f64, sigma: f64, d1: f64) -> f64 {
    let arg1 = E.powf(-(q * t)) / (s0 * sigma * (t.sqrt()));
    let arg2 = one_over_sqrt_pi();
    let arg3 = E.powf((-d1).powf(2.0)) / 2.0;
    return arg1 * arg2 * arg3;
}

#[cfg(test)]
mod tests {

    use greeks::*;

    const UNDERLYING: f64 = 64.68;
    const STRIKE: f64 = 65.00;
    const VOL: f64 = 0.5051;
    const INTEREST_RATE: f64 = 0.0150;
    const DIV_YIELD: f64 = 0.0210;
    const DAYS_PER_YEAR: f64 = 365.0;
    const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

    const E_GAMMA: f64 = 0.0243;

    #[test]
    fn test_gamma() {
        let gamma = gamma(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (gamma - E_GAMMA).abs();
        assert!(abs < 0.001);
    }
}
