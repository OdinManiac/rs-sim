pub mod optimizers;
pub mod policies;
pub mod scenario;
pub mod simulator;
pub mod systems;
extern crate nalgebra as na;

fn main() {
    let n_iterations = 10;
    let n_episodes = 2;
    let n_steps = 1000;
    let n_threads = 4;
    let initial_state = na::DVector::from_vec(vec![3.14, 0.0]);
    let dt = 0.0001;
    let mut system = systems::InvertedPendulum::default();
    let mut simulator = simulator::Simulator::new(n_steps, system, initial_state, dt);
    let policy = policies::ConstantPolicy {
        action: na::DVector::from_vec(vec![0.0]),
    };
    let optimizer = optimizers::SGD {
        learning_rate: 0.01,
        momentum: 0.0,
    };
    let mut scenario = scenario::ScenarioMC::new(
        n_iterations,
        n_episodes,
        n_steps,
        n_threads,
        simulator,
        policy,
        optimizer,
    );
}
