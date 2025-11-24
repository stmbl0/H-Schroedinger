use hydrogen_orb_core;

fn main() {
    let base_params = hydrogen_orb_core::solver::Params {
        E: 0.0, // wird überschrieben
        n: 1,
        l: 0,
        m: 0,
    };

    let opt = hydrogen_orb_core::solver::SolverOptions {
        r_start: 1e-4 * hydrogen_orb_core::constants::a0,
        r_end: 100.0 * hydrogen_orb_core::constants::a0,
        step_size: 0.00025 * hydrogen_orb_core::constants::a0,
        ψ0: 0.1,
        ψ_prime0: 0.00,
    };

    let e_low = -13.5 * hydrogen_orb_core::constants::eV;
    let e_high = -13.7 * hydrogen_orb_core::constants::eV;

    let eig = hydrogen_orb_core::energy_levels::find_eigen_energy_bisection(
        e_low,
        e_high,
        base_params,
        opt,
        1e-8 * hydrogen_orb_core::constants::eV, // tolerance
        60,                                      // max iterations
    );

    match eig {
        Ok(e) => println!("Eigen energy: {} eV", e / hydrogen_orb_core::constants::eV),
        Err(err) => eprintln!("Error: {err}"),
    }
}
