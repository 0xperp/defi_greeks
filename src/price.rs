use std::f64::consts::E;

use common::*;
use stats::cnd;

/// Evaluates the price of a European call option on an underlying which does not pay dividends before expiry of the option using the Black-Scholes model
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn euro_call(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let d2 = d2_d1(t, sigma, d1);
    let arg1 = s0 * cnd(d1);
    let arg2 = x * E.powf(-r * t) * cnd(d2);
    return arg1 - arg2;
}

/// Evaluate the price of a European put option on an underlying which does not pay dividents before expiry of the option using the Black-Scholes model
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn euro_put(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let d2 = d2_d1(t, sigma, d1);
    let arg1 = s0 * cnd(-d1);
    let arg2 = x * E.powf(-r * t) * cnd(-d2);
    return -arg1 + arg2;
}

#[cfg(test)]
mod tests {

    use price::*;

    const UNDERLYING: f64 = 64.68;
    const STRIKE: f64 = 65.00;
    const VOL: f64 = 0.5051;
    const INTEREST_RATE: f64 = 0.0150;
    const DIV_YIELD: f64 = 0.0210;
    const DAYS_PER_YEAR: f64 = 365.0;
    const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

    const E_EURO_CALL_PRICE: f64 = 3.148;
    const E_EURO_PUT_PRICE: f64 = 3.406;

    #[test]
    fn test_euro_call() {
        let price = euro_call(UNDERLYING,
                              STRIKE,
                              TIME_TO_EXPIRY,
                              INTEREST_RATE,
                              DIV_YIELD,
                              VOL);
        let abs = (price - E_EURO_CALL_PRICE).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_euro_put() {
        let price = euro_put(UNDERLYING,
                             STRIKE,
                             TIME_TO_EXPIRY,
                             INTEREST_RATE,
                             DIV_YIELD,
                             VOL);
        let abs = (price - E_EURO_PUT_PRICE).abs();
        assert!(abs < 0.001);
    }

}