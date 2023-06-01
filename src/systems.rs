extern crate nalgebra as na;
use std::collections::HashMap;

pub trait ControlledDynSystem {
    fn new(
        dim_state: usize,
        dim_action: usize,
        dim_observation: usize,
        parameters: HashMap<String, f32>,
        action: na::DVector<f32>,
    ) -> Self;
    fn reset(&mut self);
    fn compute_state_dynamics(
        &self,
        state: &na::DVector<f32>,
        action: &na::DVector<f32>,
    ) -> na::DVector<f32>;
    fn compute_closed_loop_dynamics(&self, state: &na::DVector<f32>) -> na::DVector<f32>;
    fn get_observation<'a>(&self, state: &'a na::DVector<f32>) -> &'a na::DVector<f32>;
}

#[allow(unused_variables)]
pub struct InvertedPendulum {
    dim_state: usize,
    dim_action: usize,
    dim_observation: usize,
    parameters: HashMap<String, f32>,
    action: na::DVector<f32>,
}

impl ControlledDynSystem for InvertedPendulum {
    fn new(
        dim_state: usize,
        dim_action: usize,
        dim_observation: usize,
        parameters: HashMap<String, f32>,
        action: na::DVector<f32>,
    ) -> Self {
        Self {
            dim_state,
            dim_action,
            dim_observation,
            parameters,
            action,
        }
    }
    fn reset(&mut self) {
        self.action = na::DVector::zeros(self.dim_action);
    }
    fn compute_state_dynamics(
        &self,
        state: &na::DVector<f32>,
        action: &na::DVector<f32>,
    ) -> na::DVector<f32> {
        let phi = state[0];
        let phi_dot = state[1];
        let mut derivative = na::DVector::zeros(self.dim_state);
        let m = self.parameters.get("m").unwrap();
        let g = self.parameters.get("g").unwrap();
        let l = self.parameters.get("l").unwrap();
        let F = action[0];
        derivative[0] = phi_dot;
        derivative[1] = g / l * phi.sin() + F / (m * l * l);
        return derivative;
    }
    fn compute_closed_loop_dynamics(&self, state: &na::DVector<f32>) -> na::DVector<f32> {
        self.compute_state_dynamics(state, &self.action)
    }
    fn get_observation<'a>(&self, state: &'a na::DVector<f32>) -> &'a na::DVector<f32> {
        state
    }
}

impl Default for InvertedPendulum {
    fn default() -> Self {
        Self {
            dim_state: 2,
            dim_action: 1,
            dim_observation: 0,
            parameters: HashMap::from([
                ("m".to_string(), 0.5),
                ("g".to_string(), 9.81),
                ("l".to_string(), 1.0),
            ]),
            action: na::DVector::zeros(1),
        }
    }
}
