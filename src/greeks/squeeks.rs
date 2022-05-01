// Module containing functions for calculating squeeks
// derived from squeethlab https://medium.com/opyn/how-to-think-about-squeeth-returns-8646fd57f559

// Squeeth Constants
const funding_period: f64 = 17.5 / 365.0;
const scaling_factor: f64 = 10000.0;
const eulers_number: f64 = 2.718281828459;

/// Calculates squeeth price in USD 
/// 
/// # Arguments
/// * `ETH Price` - ETH price in USD
/// * `Normalization Factor` - Normalization factor for the underlying asset
/// * `IV` - Implied volatility
///     (see https://dune.com/queries/545015/1097699 for calculating based on norm factor differences)
/// # Return
/// * sqth price
pub fn sqth_to_usd(eth_price: f64, normalization_factor: f64, iv: f64) -> f64 {
    let sqth_price = (normalization_factor * (eth_price.powf(2.0))) * eulers_number.powf(iv.powf(2.0) * funding_period) / scaling_factor;
    sqth_price
}

/// Calculates delta of a sqth position
/// 
/// # Arguments
/// * `ETH Price` - ETH price in USD
/// * `Normalization Factor` - Normalization factor for the underlying asset
/// * `IV` - Implied volatility
///     (see https://dune.com/queries/545015/1097699 for calculating based on norm factor differences)
/// # Return
/// * delta
pub fn sqth_delta(eth_price: f64, normalization_factor: f64, iv: f64) -> f64 {
    let delta = 2.0 * normalization_factor * eth_price * eulers_number.powf(iv.powf(2.0) * funding_period) / scaling_factor;
    delta
}

/// Calculates gamma of a sqth position
/// 
/// # Arguments
/// * `Normalization Factor` - Normalization factor for the underlying asset
/// * `IV` - Implied volatility
///     (see https://dune.com/queries/545015/1097699 for calculating based on norm factor differences)
/// # Return
/// * gamma
pub fn sqth_gamma(normalization_factor: f64, iv: f64) -> f64 {
    let gamma = 2.0 * normalization_factor * eulers_number.powf(iv.powf(2.0) * funding_period) / scaling_factor;
    gamma
}

/// Calculates theta of a sqth position / yr
/// 
/// # Arguments
/// * `ETH Price` - ETH price in USD
/// * `Normalization Factor` - Normalization factor for the underlying asset
/// * `IV` - Implied volatility
///     (see https://dune.com/queries/545015/1097699 for calculating based on norm factor differences)
/// # Return
/// * theta
pub fn sqth_theta(eth_price: f64, normalization_factor: f64, iv: f64) -> f64 {
    let theta = iv.powf(2.0) * sqth_to_usd(eth_price, normalization_factor, iv);
    theta
}

/// Calculates vega of a sqth position / yr
/// 
/// # Arguments
/// * `ETH Price` - ETH price in USD
/// * `Normalization Factor` - Normalization factor for the underlying asset
/// * `IV` - Implied volatility
///     (see https://dune.com/queries/545015/1097699 for calculating based on norm factor differences)
/// # Return
/// * vega
pub fn sqth_vega(eth_price: f64, normalization_factor: f64, iv: f64) -> f64 {
    let vega = 2.0 * iv * funding_period * sqth_to_usd(eth_price, normalization_factor, iv);
    vega
}

#[cfg(test)]
mod tests {
    use greeks::*;

    const eth_price: f64 = 3500.0;
    const normalization_factor: f64 = 0.8;
    const iv: f64 = 0.9;

    const E_SQTH_TO_USD: f64 = 1018.807585;
    const E_DELTA: f64 = 0.5821757629;
    const E_GAMMA: f64 = 0.0001663359322;
    const E_THETA: f64 = 825.2341438;
    const E_VEGA: f64 = 87.92449021;

    #[test]
    fn test_sqth_to_usd() {
        let sqth_to_usd = sqth_to_usd(
            eth_price,
            normalization_factor,
            iv
        );
        let abs = (sqth_to_usd - E_SQTH_TO_USD).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_sqth_delta() {
        let delta = sqth_delta(
            eth_price,
            normalization_factor,
            iv
        );
        let abs = (delta - E_DELTA).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_sqth_gamma() {
        let gamma = sqth_gamma(
            normalization_factor,
            iv
        );
        let abs = (gamma - E_GAMMA).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_sqth_theta() {
        let theta = sqth_theta(
            eth_price,
            normalization_factor,
            iv
        );
        let abs = (theta - E_THETA).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_sqth_vega() {
        let vega = sqth_vega(
            eth_price,
            normalization_factor,
            iv
        );
        let abs = (vega - E_VEGA).abs();
        assert!(abs < 0.001);
    }
}