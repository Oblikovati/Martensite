//! Global optimization settings.
use crate::math_utils::Config;
/// Global method selection; `GlobalStrategy::PSOHybrid`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlobalStrategy {
    /// Swarm-only strategy.
    PSO,
    /// Repeated Powell strategy.
    MultiStart,
    /// Swarm followed by Powell.
    #[default]
    PSOHybrid,
    /// Differential evolution strategy.
    DifferentialEvolution,
}
/// Global optimizer configuration; `GlobalConfig::default()`.
#[derive(Clone, Copy, Debug)]
pub struct GlobalConfig {
    /// Solver field; `value.solver`.
    pub solver: Config,
    /// Use_bounds field; `value.use_bounds`.
    pub use_bounds: bool,
    /// Strategy field; `value.strategy`.
    pub strategy: GlobalStrategy,
    /// Population field; `value.population`.
    pub population: usize,

    /// Starts field; `value.starts`.
    pub starts: usize,
    /// Mutation_scale field; `value.mutation_scale`.
    pub mutation_scale: f64,
    /// Crossover_prob field; `value.crossover_prob`.
    pub crossover_prob: f64,
    /// Seed field; `value.seed`.
    pub seed: u32,
    /// Polish_budget_per_dim field; `value.polish_budget_per_dim`.
    pub polish_budget_per_dim: u32,
}
impl Default for GlobalConfig {
    fn default() -> Self {
        Self::new(GlobalStrategy::PSOHybrid, 200)
    }
}
impl GlobalConfig {
    /// Uses tolerances and population defaults; `GlobalConfig::new(GlobalStrategy::PSO,200)`.
    pub fn new(strategy: GlobalStrategy, limit: u32) -> Self {
        Self {
            solver: Config::new(1e-8, limit),
            use_bounds: true,
            strategy,
            population: 40,
            starts: 10,
            mutation_scale: 0.8,
            crossover_prob: 0.9,
            seed: 6,
            polish_budget_per_dim: 50,
        }
    }
    pub(super) fn local(&self, iterations: u32) -> Config {
        Config {
            tolerance: self.solver.tolerance,
            x_tolerance: self.solver.tolerance,
            f_tolerance: self.solver.tolerance,
            max_iterations: iterations,
            ..Default::default()
        }
    }
}
