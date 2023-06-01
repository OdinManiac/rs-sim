use crate::optimizers::Optimizer;
use crate::policies::Policy;
use crate::simulator::Simulator;
use crate::systems::ControlledDynSystem;
extern crate nalgebra as na;

pub struct ScenarioMC<T, P, O>
where
    T: ControlledDynSystem,
    P: Policy,
    O: Optimizer,
{
    n_iterations: usize,
    n_episodes: usize,
    n_steps: usize,
    n_threads: usize,
    simulator: Simulator<T>,
    policy: P,
    optimizer: O,
}

impl<T, P, O> ScenarioMC<T, P, O>
where
    T: ControlledDynSystem,
    P: Policy,
    O: Optimizer,
{
    pub fn new(
        n_iterations: usize,
        n_episodes: usize,
        n_steps: usize,
        n_threads: usize,
        simulator: Simulator<T>,
        policy: P,
        optimizer: O,
    ) -> Self {
        Self {
            n_iterations,
            n_episodes,
            n_steps,
            n_threads,
            simulator,
            policy,
            optimizer,
        }
    }
}
