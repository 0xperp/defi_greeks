// Module containing functions for calculating first-order greeks
use std::f64::consts::E;

use common::*;
use stats::cnd;

/// Calculates the delta of a call option.
///
/// Delta measures the rate of the theoretical option value with respect to the changes in the underlying asset's price.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn delta_call(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let cnd = cnd(d1);
    let e = E.powf(-(q * t));
    return e * cnd;
}

/// Calculates the delta of a put options
///
/// Delta measures the rate of the theoretical option value with respect to the changes in the underlying asset's price.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn delta_put(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let cnd = cnd(d1);
    let e = E.powf(-(q * t));
    return e * (cnd - 1.0);
}

/// Calculates the lambda of a call option, also known as Omega
///
/// Omega is the percentage of change in an option's value with respect to the percentage change in the underlying price.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
/// * `v` - value or current price of the option
pub fn lambda_call(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64, v: f64) -> f64 {
    let delta = delta_call(s0, x, t, r, q, sigma);
    return lambda(s0, v, delta);
}

/// Calculates the lambda of a put option, also known as Omega
///
/// Omega is the percentage of change in an option's value with respect to the percentage change in the underlying price.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
/// * `v` - value or current price of the option
pub fn lambda_put(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64, v: f64) -> f64 {
    let delta = delta_put(s0, x, t, r, q, sigma);
    return lambda(s0, v, delta);
}

fn lambda(s0: f64, v: f64, delta: f64) -> f64 {
    return delta * s0 / v;
}

/// Calculates the Rho of a call option
///
/// Rho measures the sensitivity to the interest rate. Rho is the derivative of the option value with respect to the risk free interest rate.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn rho_call(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d2_cnd = cnd(d2(s0, x, t, r, q, sigma));
    return (1.0 / 100.0) * x * t * E.powf(-r * t) * d2_cnd;
}

/// Calculates the Rho of a put option
///
/// Rho measures the sensitivity to the interest rate. Rho is the derivative of the option value with respect to the risk free interest rate.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn rho_put(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let neg_d2_cnd = cnd(-d2(s0, x, t, r, q, sigma));
    return -(1.0 / 100.0) * x * t * E.powf(-r * t) * neg_d2_cnd;
}

/// Calculates the Theta of a call option
///
/// Theta measures the sensitivity of the value of the derivative to the passage of time.
pub fn theta_call(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64, days_per_year: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let arg1 = theta_arg_1(s0, t, q, sigma, d1);
    let d2 = d2_d1(t, sigma, d1);
    let arg2 = theta_arg_2(x, t, r, d2);
    let arg3 = theta_arg_3(s0, t, q, d1);
    return (1.0 / days_per_year) * (arg1 - arg2 + arg3);
}

/// Calculates the Theta of a put option
///
/// Theta measures the sensitivity of the value of the derivative to the passage of time.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
/// * `days_per_year` - the number of calendar days in the year
pub fn theta_put(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64, days_per_year: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    let arg1 = theta_arg_1(s0, t, q, sigma, d1);
    let d2 = d2_d1(t, sigma, d1);
    let arg2 = theta_arg_2(x, t, r, -d2); // d2 is negative for a put
    let arg3 = theta_arg_3(s0, t, q, -d1); // d1 is negative for a put
    return (1.0 / days_per_year) * (arg1 + arg2 - arg3);
}

fn theta_arg_1(s0: f64, t: f64, q: f64, sigma: f64, d1: f64) -> f64 {
    return -(((s0 * sigma * E.powf(-q * t)) / (2.0 * t.sqrt()))
        * one_over_sqrt_pi()
        * E.powf((-d1.powf(2.0)) / 2.0));
}

fn theta_arg_2(x: f64, t: f64, r: f64, d2: f64) -> f64 {
    return r * x * E.powf(-r * t) * cnd(d2);
}

fn theta_arg_3(s0: f64, t: f64, q: f64, d1: f64) -> f64 {
    return q * s0 * E.powf(-q * t) * cnd(d1);
}

/// Calculates the Vega of a given option
///
/// Vega measures the sensitivity to volatility. Vega is the derivative of the option value with respect to the volatility of the underlying asset.
///
/// # Arguments
/// * `s0` - The underlying price of the option
/// * `x` - The strike price of the option
/// * `t` - time to expiration as a percentage of the year
/// * `r` - continuously compounded risk-free interest rate
/// * `q` - continuously compounded divident yield
/// * `sigma` - volatility
pub fn vega(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    return vega_d1(s0, t, q, d1);
}

pub fn vega_d1(s0: f64, t: f64, q: f64, d1: f64) -> f64 {
    let mult1 = (1.0 / 100.0) * s0 * E.powf(-(q * t)) * t.sqrt();
    let mult2 = one_over_sqrt_pi();
    let mult3 = E.powf(-d1.powf(2.0) / 2.0);
    return mult1 * mult2 * mult3;
}

#[cfg(test)]
mod tests {

    use greeks::*;
    use value::*;

    const UNDERLYING: f64 = 64.68;
    const STRIKE: f64 = 65.00;
    const VOL: f64 = 0.5051;
    const INTEREST_RATE: f64 = 0.0150;
    const DIV_YIELD: f64 = 0.0210;
    const DAYS_PER_YEAR: f64 = 365.0;
    const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

    const E_CALL_DELTA: f64 = 0.5079;
    const E_PUT_DELTA: f64 = -0.4908;
    const E_LAMBDA_PUT: f64 = -3.0759;
    const E_LAMBDA_CALL: f64 = 3.3936;
    const E_RHO_CALL: f64 = 0.0187;
    const E_RHO_PUT: f64 = -0.0222;
    const E_THETA_CALL: f64 = -0.0703;
    const E_THETA_PUT: f64 = -0.0714;
    const E_VEGA: f64 = 0.0647;

    #[test]
    fn test_delta_call() {
        let call_delta = delta_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (call_delta - E_CALL_DELTA).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_delta_put() {
        let put_delta = delta_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (put_delta - E_PUT_DELTA).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_lambda_put() {
        // Abitrary change in underlying at expiry
        let price = put_at_expiry(UNDERLYING - 10.0, STRIKE);
        let lambda = lambda_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            price,
        );
        println!("{}", lambda);
        let abs = (lambda - E_LAMBDA_PUT).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_lambda_call() {
        // abitrary change in underlying at expiry
        let price = call_at_expiry(UNDERLYING + 10.0, STRIKE);
        let lambda = lambda_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            price,
        );
        let abs = (lambda - E_LAMBDA_CALL).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_rho_call() {
        let rho_call = rho_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (rho_call - E_RHO_CALL).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_rho_put() {
        let rho_put = rho_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (rho_put - E_RHO_PUT).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_theta_call() {
        let theta_call = theta_call(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            DAYS_PER_YEAR,
        );
        let abs = (theta_call - E_THETA_CALL).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_theta_put() {
        let theta_put = theta_put(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
            DAYS_PER_YEAR,
        );
        let abs = (theta_put - E_THETA_PUT).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_vega() {
        let vega = vega(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (vega - E_VEGA).abs();
        assert!(abs < 0.001);
    }
}
