/*
mod core:

the core of the renderer; contains all the physics data.
uses dynamics_calculator to update the physics data state.
uses initiator to help initialize the physics data.
*/

use std::time::{Duration, SystemTime};

use crate::physics::interface::{initiator};
use nalgebra as na;

pub struct Core {
    pub physics_data: PhysicsData,
    last_iteration_start_time: SystemTime,
}

impl Core {
    pub fn new() -> Self {
        Self{
            physics_data: PhysicsData::new(),
            last_iteration_start_time: SystemTime::now(),
        }
    }
    pub fn initialize(&mut self) {
        initiator::initialize(&mut self.physics_data);
    }
    pub fn run(&mut self) {
        loop {
            self.wait_delta_time();
            self.run_iteration();
        }
    }
    fn wait_delta_time(&mut self) {
        const DELTA_TIME: f64 = 0.1;
        let current_time = SystemTime::now();
        let next_iteration_start_time = self.last_iteration_start_time + Duration::from_secs_f64(DELTA_TIME);
        match next_iteration_start_time.duration_since(current_time) {
            Ok(duration) => {std::thread::sleep(duration)}
            Err(_) => {}
        };
        self.last_iteration_start_time = next_iteration_start_time;
    }
    fn run_iteration(&mut self) {
        println!("iteration");
    }
}

pub struct PhysicsData {
    pub particles: Vec<Particle>,

}

impl PhysicsData {
    pub fn new() -> Self {
        Self{
            particles: vec![Particle::new(); 1]
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub position: na::Vector3<f64>,
    pub velocity: na::Vector3<f64>,
    pub mass: f64,
}

impl Particle {
    pub fn new() -> Self {
        Self {
            position: na::Vector3::new(0.0, 0.0, 0.0),
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            mass: 1.0,
        }
    }
    pub fn iterate(&mut self, delta: f64) {
        self.apply_gravity(delta);
        self.integrate_position(delta);
    }
}