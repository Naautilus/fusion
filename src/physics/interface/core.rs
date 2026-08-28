mod dynamics_calculator;
mod initiator;

/*
mod core:

the core of the renderer; contains all the physics data.
uses dynamics_calculator to update the physics data state.
uses initiator to help initialize the physics data.
*/

pub struct Core {
    pub physics_data: PhysicsData,
}

impl Core {
    pub fn new() -> Self {
        Self{physics_data: PhysicsData::new()}
    }
    pub fn initialize(&mut self) {
        initiator::initialize(&mut self.physics_data);
    }
}

pub struct PhysicsData {
    pub data_blablabla: i32,

}

impl PhysicsData {
    pub fn new() -> Self {
        Self{data_blablabla: 0}
    }
}

pub struct Particle {
    position: na::Vector3<f64>,
    velocity: na::Vector3<f64>,
    mass: na::Vector3<f64>,
}
