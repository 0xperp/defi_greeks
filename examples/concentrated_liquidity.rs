extern crate greeks;

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

fn main() {
    let virtual_liquidity = greeks::virtual_liquidity(
        P_A, P_B, R_B,
        R_A, // note: its tough to figure out which token is what you are pricing your greeks in, sometimes you might need to switch them around
    );

    println!("Expected virtual liquidity: {:2.2}", E_RV);
    println!("Actual virtual liquidity: {:2.2}", virtual_liquidity);

    println!("{}, {}, {}", virtual_liquidity, P, P_B);
    let delta = greeks::concentrated_delta(virtual_liquidity, P, P_B);
    let gamma = greeks::concentrated_gamma(virtual_liquidity, P);

    println!("Expected delta: {:2}", E_DELTA);
    println!("Actual delta: {:2}", delta);

    println!("Expected gamma: {:2}", E_GAMMA);
    println!("Actual gamma: {:2}", gamma);
}
