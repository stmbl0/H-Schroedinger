use super::constants;
use std::f64::consts;

/// Calculates the Coulomb Potential for a specified radius from the hydrogen atom's core
pub fn coulomb_potential(r: f64) -> f64 {
    return -(constants::e.powf(4.0) / (4.0 * consts::PI * constants::ε0)) * (1.0 / r);
}
