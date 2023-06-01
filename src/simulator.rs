extern crate nalgebra as na;
use crate::systems::ControlledDynSystem;
pub struct Simulator<T>
where
    T: ControlledDynSystem,
{
    n_steps: usize,
    step_number: usize,
    system: T,
    state: na::DVector<f32>,
    initial_state: na::DVector<f32>,
    time: f32,
    dt: f32,
}
pub enum SimState {
    FINISHED,
    RUNNING,
}

impl<T> Simulator<T>
where
    T: ControlledDynSystem,
{
    pub fn new(n_steps: usize, system: T, initial_state: na::DVector<f32>, dt: f32) -> Self {
        let state = initial_state.clone();
        Self {
            n_steps,
            step_number: 0,
            system,
            state,
            initial_state,
            time: 0.0,
            dt,
        }
    }
    pub fn reset(&mut self) {
        self.state = self.initial_state.clone();
        self.time = 0.0;
        self.step_number = 0;
        self.system.reset();
    }
    pub fn step(&mut self) -> SimState {
        self.time += self.dt;
        self.step_number += 1;
        let derivative = self.system.compute_closed_loop_dynamics(&self.state);
        self.state += derivative * self.dt;
        match self.step_number >= self.n_steps {
            true => {
                self.reset();
                SimState::FINISHED
            }
            false => SimState::RUNNING,
        }
    }
    pub fn get_sim_step_data(&self) -> (&na::DVector<f32>, f32) {
        (&self.state, self.time)
    }
    pub fn receive_action(&mut self, action: &na::DVector<f32>) {
        self.system.receive_action(action);
    }
}
