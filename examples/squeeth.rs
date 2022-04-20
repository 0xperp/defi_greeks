use greeks::sqth_theta;

extern crate greeks;

const eth_price: f64 = 3500.0;
const normalization_factor: f64 = 0.8;
const iv: f64 = 0.9;

fn main() {
    let sqth_delta = greeks::sqth_delta(
        eth_price,
        normalization_factor,
        iv
    );
    let sqth_gamma = greeks::sqth_gamma(
        normalization_factor,
        iv
    );
    let sqth_theta = greeks::sqth_theta(
        eth_price,
        normalization_factor,
        iv
    );
    let sqth_vega = greeks::sqth_vega(
        eth_price,
        normalization_factor,
        iv
    );

    println!("Delta: {:2.2}", sqth_delta);
    println!("Gamma: {:2}", sqth_gamma);
    println!("Theta: {:2.2}", sqth_theta);
    println!("Vega: {:2.2}", sqth_vega);
}
