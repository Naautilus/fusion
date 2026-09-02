mod core;
mod dynamics;
mod initiator;

/*
physics::interface:

has functions that help start the physics engine, and manages the giving of simulation data to the renderer thread.
*/



pub struct Interface {
    core_: core::Core,
}

impl Interface {
    pub fn initialize_physics_engine() -> Self {
        let mut core__ = core::Core::new();
        core__.initialize();
        Self{core_: core__}
    }
    pub fn run_physics_loop(&mut self) {
        self.core_.run();
    }
}