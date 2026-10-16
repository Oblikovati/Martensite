//! Particle swarm optimization settings.
use crate::math::Vector;
use crate::math_utils::Config;
/// Seed selection policy; `PSOInitMode::RandomOnly`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PSOInitMode {
    #[default]
    /// RandomOnly policy.
    RandomOnly,
    /// SeededOnly policy.
    SeededOnly,
    /// SeededPlusRandom policy.
    SeededPlusRandom,
}
/// Boundary correction policy; `PSOBoundaryMode::Clamp`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PSOBoundaryMode {
    #[default]
    /// Clamp policy.
    Clamp,
    /// Reflect policy.
    Reflect,
    /// Wrap policy.
    Wrap,
}
/// Inertia policy; `PSOInertiaSchedule::Constant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PSOInertiaSchedule {
    #[default]
    /// Constant policy.
    Constant,
    /// LinearDecay policy.
    LinearDecay,
}
/// Seeds retain advisory fitness, but always evaluate the callback; `PSOSeedParticle::new(&position)`.
pub struct PSOSeedParticle {
    /// Position field; `value.position`.
    pub position: Vector<'static>,
    /// Value field; `value.value`.
    pub value: Option<f64>,
    /// Velocity field; `value.velocity`.
    pub velocity: Option<Vector<'static>>,
}
impl PSOSeedParticle {
    /// Copies the input position with its bounds; `PSOSeedParticle::new(&position)`.
    pub fn new(position: &Vector<'_>) -> Self {
        {
            let mut copied: Vector<'static> =
                Vector::with_bounds(position.lower(), position.upper()).unwrap();
            copied.initialized(position).unwrap();
            Self {
                position: copied,
                value: None,
                velocity: None,
            }
        }
    }
    /// Stores an advisory fitness; `PSOSeedParticle::with_value(&position, 1.0)`.
    pub fn with_value(position: &Vector<'_>, value: f64) -> Self {
        Self {
            value: Some(value),
            ..Self::new(position)
        }
    }
    pub(super) fn valid(&self, n: usize) -> bool {
        self.position.length() as usize == n
            && self.position.values().iter().all(|v| v.is_finite())
            && self.value.is_none_or(f64::is_finite)
            && self.velocity.as_ref().is_none_or(|v| {
                v.length() as usize == n && v.values().iter().all(|x| x.is_finite())
            })
    }
}
/// Execution counters; `PSOStats::default()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct PSOStats {
    /// Function_evals field; `value.function_evals`.
    pub function_evals: usize,
    /// Iterations field; `value.iterations`.
    pub iterations: u32,
    /// Boundary_corrections field; `value.boundary_corrections`.
    pub boundary_corrections: usize,

    /// Stagnation_events field; `value.stagnation_events`.
    pub stagnation_events: usize,
    /// Restarts field; `value.restarts`.
    pub restarts: usize,
    /// Initial_best field; `value.initial_best`.
    pub initial_best: Option<f64>,
    /// Final_best field; `value.final_best`.
    pub final_best: Option<f64>,
}
/// Particle swarm settings; `PSOConfig::default()`.
#[derive(Clone, Copy, Debug)]
pub struct PSOConfig {
    /// Solver field; `value.solver`.
    pub solver: Config,
    /// Use_bounds field; `value.use_bounds`.
    pub use_bounds: bool,
    /// Particles field; `value.particles`.
    pub particles: usize,
    /// Omega field; `value.omega`.
    pub omega: f64,

    /// Phi_personal field; `value.phi_personal`.
    pub phi_personal: f64,
    /// Phi_global field; `value.phi_global`.
    pub phi_global: f64,
    /// Velocity_clamp field; `value.velocity_clamp`.
    pub velocity_clamp: f64,
    /// Seed field; `value.seed`.
    pub seed: u32,

    /// Init_mode field; `value.init_mode`.
    pub init_mode: PSOInitMode,
    /// Boundary_mode field; `value.boundary_mode`.
    pub boundary_mode: PSOBoundaryMode,
    /// Inertia_schedule field; `value.inertia_schedule`.
    pub inertia_schedule: PSOInertiaSchedule,

    /// Omega_min field; `value.omega_min`.
    pub omega_min: f64,
    /// Min_iterations field; `value.min_iterations`.
    pub min_iterations: u32,
    /// Target_value field; `value.target_value`.
    pub target_value: Option<f64>,

    /// No_improve_tol field; `value.no_improve_tol`.
    pub no_improve_tol: f64,
    /// No_improve_iters field; `value.no_improve_iters`.
    pub no_improve_iters: u32,
    /// Restart_fraction field; `value.restart_fraction`.
    pub restart_fraction: f64,

    /// Max_restarts field; `value.max_restarts`.
    pub max_restarts: u32,
    /// Polish_budget_per_dim field; `value.polish_budget_per_dim`.
    pub polish_budget_per_dim: u32,
}
impl Default for PSOConfig {
    fn default() -> Self {
        Self::new(40, 100, 1e-8)
    }
}
impl PSOConfig {
    /// Sets population, limit and tolerances; `PSOConfig::new(40,100,1e-8)`.
    pub fn new(particles: usize, limit: u32, tolerance: f64) -> Self {
        Self {
            solver: Config::new(tolerance, limit),
            use_bounds: true,
            particles,
            omega: 0.7,
            phi_personal: 1.5,
            phi_global: 1.5,
            velocity_clamp: 0.5,
            seed: 6,
            init_mode: PSOInitMode::RandomOnly,
            boundary_mode: PSOBoundaryMode::Clamp,
            inertia_schedule: PSOInertiaSchedule::Constant,
            omega_min: 0.4,
            min_iterations: 0,
            target_value: None,
            no_improve_tol: 0.0,
            no_improve_iters: 10,
            restart_fraction: 0.0,
            max_restarts: 0,
            polish_budget_per_dim: 50,
        }
    }
    pub(super) fn valid(&self) -> bool {
        super::super::utils::is_valid_config(&self.solver)
            && self.particles > 0
            && self.omega.is_finite()
            && self.omega_min.is_finite()
            && [
                self.phi_personal,
                self.phi_global,
                self.velocity_clamp,
                self.no_improve_tol,
                self.restart_fraction,
            ]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            && self.min_iterations <= self.solver.max_iterations
            && self.no_improve_iters > 0
            && self.restart_fraction <= 1.0
            && self.target_value.is_none_or(f64::is_finite)
    }
    pub(super) fn inertia(&self, iteration: u32) -> f64 {
        if self.inertia_schedule == PSOInertiaSchedule::LinearDecay {
            self.omega
                - (self.omega - self.omega_min) * iteration as f64
                    / self.solver.max_iterations as f64
        } else {
            self.omega
        }
    }
}
