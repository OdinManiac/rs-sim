extern crate nalgebra as na;

pub trait Optimizer {
    fn optimize(&self, state: &na::DVector<f32>) -> na::DVector<f32>;
}

pub struct SGD {
    pub learning_rate: f32,
    pub momentum: f32,
}

impl Optimizer for SGD {
    fn optimize(&self, state: &na::DVector<f32>) -> na::DVector<f32> {
        na::DVector::zeros(state.len())
    }
}
