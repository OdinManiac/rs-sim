extern crate nalgebra as na;

pub trait Policy {
    fn choose_action(&self, state: &na::DVector<f32>) -> &na::DVector<f32>;
}

pub struct ConstantPolicy {
    pub action: na::DVector<f32>,
}

#[allow(unused_variables)]
impl Policy for ConstantPolicy {
    fn choose_action(&self, state: &na::DVector<f32>) -> &na::DVector<f32> {
        &self.action
    }
}
