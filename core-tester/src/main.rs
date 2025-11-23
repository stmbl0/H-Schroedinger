use hydrogen_orb_core;

fn main() {
    for a in 1360500..1360800 {
        test_E(-((a as f64) / 100000.0) * hydrogen_orb_core::constants::eV);
    }
}

fn test_E(E: f64) {
    let result = hydrogen_orb_core::solver::solve(
        hydrogen_orb_core::solver::Params {
            E: E,
            n: 1,
            l: 0,
            m: 0,
        },
        hydrogen_orb_core::solver::SolverOptions {
            r_start: 1e-4 * hydrogen_orb_core::constants::a0,
            r_end: 50.0 * hydrogen_orb_core::constants::a0,
            step_size: 0.00025 * hydrogen_orb_core::constants::a0,
            ψ0: 1.0,
            ψ_prime0: 0.0,
        },
    );

    let mut psi_last: f64 = 0.0;

    for step in result {
        psi_last = step.1.y1;
    }
    println!("{}, {}", E / hydrogen_orb_core::constants::eV, psi_last);
}
