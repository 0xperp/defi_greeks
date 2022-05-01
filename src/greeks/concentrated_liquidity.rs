// Module containing functions for calculating greeks based on concentrated liquidity
// derived from a series of blog posts:
// https://lambert-guillaume.medium.com/understanding-the-value-of-uniswap-v3-liquidity-positions-cdaaee127fe7
// https://medium.com/opyn/hedging-uniswap-v3-with-squeeth-bcaf1750ea11
// https://papers.ssrn.com/sol3/papers.cfm?abstract_id=3898384
//    See Appendix A for details on the formula
// and code from https://github.com/guil-lambert/yewbow-info/blob/32601e63cceb16d88c846a8578a2c04d83e53ba8/src/components/DensityChart/CustomToolTip.tsx#L45
// Refer to https://gist.github.com/0xperp/fe5327d05b59c9122332d860adf2ba42 for a python notebook on the formulas

// Cavets with greeks and concentrated liquididty ranges
// Positions have no gamma outside of the range

/// Calculates virtual liquidity of a concentrated liquidity share
/// Refer to https://gist.github.com/0xperp/fe5327d05b59c9122332d860adf2ba42 for a python notebook on the formulas
/// # Arguments
/// * `p_a` - Lower Tick range
/// * `p_b` - Upper tick range
/// * `R_a` - Reserves of token a
/// * `R_b` - Reserves of token b
/// # Return
/// * virtual liquidity
pub fn virtual_liquidity(p_a: f32, p_b: f32, r_a: f32, r_b: f32) -> f32 {
    // solving "bounded liquidity position" eq. 1 for L
    // terms for quadratic eq.
    let a = (p_a.sqrt() / p_b.sqrt()) - 1_f32;
    let b = (r_b / p_b.sqrt()) + (r_a * p_a.sqrt());
    let c = r_a * r_b;

    // discriminant
    let d = b.powf(2.0) - (4_f32 * a * c);

    // solutions
    let solution1 = (-b - d.sqrt()) / (2.0 * a);
    let solution2 = (-b + d.sqrt()) / (2.0 * a);

    // virtual reserves
    let r_v = {
        if solution1 > 0.0 {
            solution1
        } else {
            solution2
        }
    };
    r_v
}

/// Calculates delta of a concentrated liquidity share
/// Refer to https://gist.github.com/0xperp/fe5327d05b59c9122332d860adf2ba42 for a python notebook on the formulas
/// # Arguments
/// * `L` - Virtual Liquidity
/// * `p_b` - Upper tick range
/// * `p` - Current price
/// # Return
/// * delta
pub fn concentrated_delta(l: f32, p: f32, p_b: f32) -> f32 {
    l * (1.0 / p.sqrt() - 1.0 / p_b.sqrt())
}

/// Calculates gamma of a concentrated liquidity share
/// Refer to https://gist.github.com/0xperp/fe5327d05b59c9122332d860adf2ba42 for a python notebook on the formulas
/// # Arguments
/// * `L` - Virtual Liquidity
/// * `p` - Current price
/// # Return
/// * gamma
pub fn concentrated_gamma(l: f32, p: f32) -> f32 {
    0.5 * l * p.powf(-1.5)
}

#[cfg(test)]
mod tests {
    use greeks::*;

    // a token reserves
    const R_A: f32 = 6779.0;
    // b token reserves
    const R_B: f32 = 1.448;
    // lower range
    const P_A: f32 = 3747.0;
    // upper range
    const P_B: f32 = 5024.0;
    // current price
    const P: f32 = 4360.61;

    // expected virtual liquididty
    const E_RV: f32 = 1402.4046379549889;
    // expected delta
    const E_DELTA: f32 = 1.4517521820181736;
    // expected gamma
    const E_GAMMA: f32 = 0.002435131811150409;

    #[test]
    fn test_virtual_liquidity() {
        let virtual_liquidity = virtual_liquidity(
            P_A, P_B, R_B,
            R_A, // note: its tough to figure out which token is what you are pricing your greeks in, sometimes you might need to switch them around
        );

        let abs = (virtual_liquidity - E_RV).abs();
        assert!(abs < 0.1);
    }

    #[test]
    fn test_delta() {
        let virtual_liquidity = virtual_liquidity(
            P_A, P_B, R_B,
            R_A, // note: its tough to figure out which token is what you are pricing your greeks in, sometimes you might need to switch them around
        );

        let delta = concentrated_delta(virtual_liquidity, P, P_B);

        let abs = (delta - E_DELTA).abs();
        assert!(abs < 0.1);
    }

    #[test]
    fn test_gamma() {
        let virtual_liquidity = virtual_liquidity(
            P_A, P_B, R_B,
            R_A, // note: its tough to figure out which token is what you are pricing your greeks in, sometimes you might need to switch them around
        );

        let gamma = concentrated_gamma(virtual_liquidity, P);

        let abs = (gamma - E_GAMMA).abs();
        assert!(abs < 0.1);
    }
}
