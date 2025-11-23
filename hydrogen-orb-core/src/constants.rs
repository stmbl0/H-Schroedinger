#![allow(nonstandard_style)]

use std::f64::consts;

/// Elementary charge [Coloumb]
pub const e: f64 = 1.602176634e-19;
/// Mass of an electron [kg]
pub const me: f64 = 9.109383713928e-31;
/// Permeability of free space [(A*s)/(V*m)]
pub const ε0: f64 = 8.854187818814e-12;
/// Planks constant [J*s]
pub const h: f64 = 6.62607015e-34;
pub const h_bar: f64 = h / (2.0 * consts::PI);
/// Speed of light in a vacuum [m/s]
pub const c: f64 = 2.99792458e8;
/// Energy equivalent of one electron-volt [Joule]
pub const eV: f64 = 1.602176634e-19;
/// Bohr radius [m]
pub const a0: f64 = 5.29177210544e-11;
