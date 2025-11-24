#![allow(nonstandard_style)]

use crate::solver;

/// Integrate for a given energy E and return ψ(r_end)
pub fn psi_at_r_end(E: f64, base_params: solver::Params, opt: solver::SolverOptions) -> f64 {
    let params = solver::Params { E, ..base_params };

    let result = solver::solve(params, opt);

    // last() is safe because solve() always pushes at least one step;
    // unwrap() panics otherwise, but in practice that's fine for now.
    result.last().unwrap().1.y1
}

/// Find an eigen energy between e_low and e_high using the shooting method + bisection.
///
/// - `e_low`, `e_high` in Joule
/// - `tol_e` as target tolerance of energy (e.g. 1e-8 * eV)
/// - `max_iter` limits the loop
pub fn find_eigen_energy_bisection(
    mut e_low: f64,
    mut e_high: f64,
    params: solver::Params,
    opt: solver::SolverOptions,
    tol_e: f64,
    max_iter: usize,
) -> Result<f64, &'static str> {
    let mut psi_low = psi_at_r_end(e_low, params, opt);
    let psi_high = psi_at_r_end(e_high, params, opt);

    if psi_low * psi_high > 0.0 {
        return Err("No sign change of ψ(r_end) in [e_low, e_high]; no eigenvalue bracketed.");
    }

    for _ in 0..max_iter {
        let e_mid = 0.5 * (e_low + e_high);
        let psi_mid = psi_at_r_end(e_mid, params, opt);

        if psi_mid.abs() < 1e-8 {
            return Ok(e_mid);
        }

        // Change of sign -> half the interval
        if psi_low * psi_mid < 0.0 {
            e_high = e_mid;
            //psi_high = psi_mid;
        } else {
            e_low = e_mid;
            psi_low = psi_mid;
        }

        // energy range small enough?
        if (e_high - e_low).abs() < tol_e {
            return Ok(0.5 * (e_low + e_high));
        }
    }

    Err("Bisection did not converge within max_iter.")
}
