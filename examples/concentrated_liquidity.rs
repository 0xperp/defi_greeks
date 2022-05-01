extern crate greeks;

// a token reserves
const R_a: f32 = 6779.0;
// b token reserves
const R_b: f32 = 1.448;
// lower range
const p_a: f32 = 3747.0;
// upper range
const p_b: f32 = 5024.0;
// current price
const p: f32 = 4360.61;

// expected virtual liquididty
const E_Rv: f32 = 1402.4046379549889;
// expected delta
const E_delta: f32 = 1.4517521820181736;
// expected gamma
const E_gamma: f32 = 0.002435131811150409;

fn main() {
    let virtual_liquidity = greeks::virtual_liquidity(
        p_a, p_b, R_b,
        R_a, // note: its tough to figure out which token is what you are pricing your greeks in, sometimes you might need to switch them around
    );

    println!("Expected virtual liquidity: {:2.2}", E_Rv);
    println!("Actual virtual liquidity: {:2.2}", virtual_liquidity);

    println!("{}, {}, {}", virtual_liquidity, p, p_b);
    let delta = greeks::concentrated_delta(virtual_liquidity, p, p_b);
    let gamma = greeks::concentrated_gamma(virtual_liquidity, p);

    println!("Expected delta: {:2}", E_delta);
    println!("Actual delta: {:2}", delta);

    println!("Expected gamma: {:2}", E_gamma);
    println!("Actual gamma: {:2}", gamma);
}
