use std::f64::consts::PI;

pub fn d1(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let ln = (s0 / x).ln();
    let t_num = t * (r - q + (sigma.powf(2f64) / 2f64));
    return (ln + t_num) / (sigma * t.sqrt());
}

pub fn d2(s0: f64, x: f64, t: f64, r: f64, q: f64, sigma: f64) -> f64 {
    let d1 = d1(s0, x, t, r, q, sigma);
    return d1 - (t.sqrt() * sigma);
}

pub fn d2_d1(t: f64, sigma: f64, d1: f64) -> f64 {
    return d1 - (t.sqrt() * sigma);
}

pub fn one_over_sqrt_pi() -> f64 {
    return 1.0 / (2.0 * PI).sqrt();
}

#[cfg(test)]
mod tests {

    use common::*;

    const UNDERLYING: f64 = 64.68;
    const STRIKE: f64 = 65.00;
    const VOL: f64 = 0.5051;
    const INTEREST_RATE: f64 = 0.0150;
    const DIV_YIELD: f64 = 0.0210;
    const DAYS_PER_YEAR: f64 = 365.0;
    const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

    const E_D1: f64 = 0.0214;
    const E_D2: f64 = -0.1053;

    #[test]
    fn test_d1() {
        let d1 = d1(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (d1 - E_D1).abs();
        assert!(abs < 0.001);
    }

    #[test]
    fn test_d2() {
        let d2 = d2(
            UNDERLYING,
            STRIKE,
            TIME_TO_EXPIRY,
            INTEREST_RATE,
            DIV_YIELD,
            VOL,
        );
        let abs = (d2 - E_D2).abs();
        assert!(abs < 0.001);
    }
}
