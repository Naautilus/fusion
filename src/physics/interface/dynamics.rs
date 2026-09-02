/*
mod dynamics:

does all the updating of the physics data state.
*/

use crate::physics::interface::core;
use nalgebra as na;

const GRAVITY: na::Vector3<f64> = na::Vector3::new(0.0, 0.0, -9.81);

impl core::Particle {
    pub fn apply_gravity(&mut self, delta: f64) {
        self.velocity += GRAVITY * delta;
    }
    pub fn integrate_position(&mut self, delta: f64) {
        self.position += self.velocity * delta;
    }
}