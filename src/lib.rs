/// Calculates the force of gravity using Newton's theory of gravity.
///
/// # Examples
///
/// ```
/// let m1 = 1000000.0;
/// let m2 = 2000000.0;
/// let r = 1.0;
/// let answer = gravity_force_rust::calc_gravity_force(m1, m2, r);
/// assert_eq!(133.4, answer);
/// ```
///
pub fn calc_gravity_force(m1: f64, m2: f64, r: f64) -> f64 {
    let g = 6.67 * f64::powf(10.0, -11.0);

    // Returns gravity value
    g * (m1 * m2) / f64::powf(r, 2.0)
}
