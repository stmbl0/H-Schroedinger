use crate::constants;
use crate::potential;

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub y1: f64, // ψ(r)
    pub y2: f64, // ψ'(r)
}

pub struct Params {
    /// Energy of the electron
    pub E: f64,
    /// Principal quantum number
    pub n: u16,
    /// Angular momentum quantum number
    pub l: u16,
    /// Magnetic quantum number
    pub m: i16,
}

pub struct SolverOptions {
    pub r_start: f64,
    pub r_end: f64,
    pub step_size: f64,
    pub ψ0: f64,
    pub ψ_prime0: f64,
}

pub fn radial_rhs(r: f64, s: State, params: &Params) -> State {
    let potential = potential::coulomb_potential(r);

    let ψ = s.y1;
    let ψ_prime = s.y2;

    let a = ((2.0 * constants::me) / (constants::h_bar * constants::h_bar))
        * (params.E
            - potential
            - ((params.l * (params.l + 1)) as f64 * constants::h_bar.powf(2.0))
                / (2.0 * constants::me * r * r));

    let ψ_second = -(2.0 / r) * ψ_prime - a * ψ;
    return State {
        y1: ψ_prime,
        y2: ψ_second,
    };
}

fn rk4_step2<F>(f: F, r: f64, s: State, h: f64) -> State
where
    F: Fn(f64, State) -> State,
{
    let k1 = f(r, s);

    let s2 = State {
        y1: s.y1 + 0.5 * h * k1.y1,
        y2: s.y2 + 0.5 * h * k1.y2,
    };
    let k2 = f(r + 0.5 * h, s2);

    let s3 = State {
        y1: s.y1 + 0.5 * h * k2.y1,
        y2: s.y2 + 0.5 * h * k2.y2,
    };
    let k3 = f(r + 0.5 * h, s3);

    let s4 = State {
        y1: s.y1 + h * k3.y1,
        y2: s.y2 + h * k3.y2,
    };
    let k4 = f(r + h, s4);

    State {
        y1: s.y1 + (h / 6.0) * (k1.y1 + 2.0 * k2.y1 + 2.0 * k3.y1 + k4.y1),
        y2: s.y2 + (h / 6.0) * (k1.y2 + 2.0 * k2.y2 + 2.0 * k3.y2 + k4.y2),
    }
}

fn integrate<F>(f: F, r_start: f64, r_end: f64, h: f64, s0: State) -> Vec<(f64, State)>
where
    F: Fn(f64, State) -> State,
{
    let mut data = Vec::new();
    let mut r = r_start;
    let mut s = s0;

    while r <= r_end {
        data.push((r, s));
        s = rk4_step2(&f, r, s, h);
        r += h;
    }

    data
}

pub fn solve(params: Params, opt: SolverOptions) -> Vec<(f64, State)> {
    let f = |r: f64, s: State| radial_rhs(r, s, &params);
    let result = integrate(
        f,
        opt.r_start,
        opt.r_end,
        opt.step_size,
        State {
            y1: opt.ψ0,
            y2: opt.ψ_prime0,
        },
    );

    return result;
}
