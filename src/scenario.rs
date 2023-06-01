use crate::optimizers::Optimizer;
use crate::policies::Policy;
use crate::simulator::{SimState, Simulator};
use crate::systems::ControlledDynSystem;
extern crate nalgebra as na;
use log::{info, warn};

pub struct ScenarioMC<'a, T, P, O>
where
    T: ControlledDynSystem,
    P: Policy,
    O: Optimizer,
{
    n_iterations: usize,
    n_episodes: usize,
    n_steps: usize,
    n_threads: usize,
    simulator: &'a mut Simulator<T>,
    policy: P,
    optimizer: O,
}

impl<'a, T, P, O> ScenarioMC<'a, T, P, O>
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
        simulator: &'a mut Simulator<T>,
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
    pub fn run(&mut self) {
        for _ in 0..self.n_iterations {
            for _ in 0..self.n_episodes {
                loop {
                    match self.simulator.step() {
                        SimState::FINISHED => break,
                        SimState::RUNNING => {
                            let (state, time) = self.simulator.get_sim_step_data();
                            let action = self.policy.choose_action(&state);
                            {
                                info!("action: {:?}, state: {:?}", action, state);
                            }
                            self.simulator.receive_action(&action);
                        }
                    }
                }
            }
        }
    }
}
