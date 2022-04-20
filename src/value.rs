/// Calculates the value of a call option at Expiry
///
/// # Arguments
///
/// `s_t` - Price of the underlying at expiry date
/// `x` - Option strike price
pub fn call_at_expiry(s_t: f64, x: f64) -> f64 {
    let res = s_t - x;
    return if res > 0.0 { res } else { 0.0 };
}

/// Calculates the value of a put option at Expiry
///
/// # Arguments
///
/// `s_t` - Price of the underlying at expiry date
/// `x` - Option strike price
pub fn put_at_expiry(s_t: f64, x: f64) -> f64 {
    let res = x - s_t;
    return if res > 0.0 { res } else { 0.0 };
}