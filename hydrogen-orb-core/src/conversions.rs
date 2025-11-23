#![allow(nonstandard_style)]
use super::constants;

/// Calculates the energy of a photon (in Joule) with a specific wavelength (in meters)
pub fn photon_λ_to_J(λ: f64) -> f64 {
    return (constants::h * constants::c) / λ;
}

/// Calculates the energy of a photon (in electron volts) with a specific wavelength (in meters)
pub fn photon_λ_to_eV(λ: f64) -> f64 {
    return energy_J_to_eV(photon_λ_to_J(λ));
}

/// Calculates the wavelength of a photon (in meters) given its energy (in Joules)
pub fn photon_J_to_λ(energy_J: f64) -> f64 {
    return (constants::h * constants::c) / energy_J;
}

/// Calculates the wavelength of a photon (in meters) given its energy (in electron volts)
pub fn photon_eV_to_λ(energy_eV: f64) -> f64 {
    return photon_J_to_λ(energy_eV_to_J(energy_eV));
}

pub fn energy_J_to_eV(energy_J: f64) -> f64 {
    return energy_J / constants::eV;
}

pub fn energy_eV_to_J(energy_eV: f64) -> f64 {
    return energy_eV * constants::eV;
}
