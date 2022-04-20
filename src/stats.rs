// Helper module containing statistics functions
use std::f64::consts::E;

const A1: f64 = 0.31938153;
const A2: f64 = -0.356563782;
const A3: f64 = 1.781477937;
const A4: f64 = -1.821255978;
const A5: f64 = 1.330274429;
const RSQRTPI: f64 = 0.39894228040143267793994605993438;

// Simple implementation of cumulative normal distribution for a provided 'x'
pub fn cnd(x: f64) -> f64 {
    let k = 1.0 / (1.0 + 0.2316419 * x.abs());
    let mut cnd: f64 = RSQRTPI * E.powf(-0.5 * x * x) *
                       (k * (A1 + k * (A2 + k * (A3 + k * (A4 + k * A5)))));
    if x > 0.0 {
        cnd = 1.0 - cnd;
    }
    return cnd;
}