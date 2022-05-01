extern crate greeks;

const UNDERLYING: f64 = 64.68;
const STRIKE: f64 = 65.00;
const VOL: f64 = 0.5051;
const INTEREST_RATE: f64 = 0.0150;
const DIV_YIELD: f64 = 0.0210;
const DAYS_PER_YEAR: f64 = 365.0;
const TIME_TO_EXPIRY: f64 = 23.0 / DAYS_PER_YEAR;

const E_CALL_DELTA: f64 = 0.5079;

fn main() {
    let actual_delta = greeks::delta_call(
        UNDERLYING,
        STRIKE,
        TIME_TO_EXPIRY,
        INTEREST_RATE,
        DIV_YIELD,
        VOL,
    );

    println!("Expected delta: {:2.2}", E_CALL_DELTA);
    println!("Actual Delta: {:2.2}", actual_delta);
}
