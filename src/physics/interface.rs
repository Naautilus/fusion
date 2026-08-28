mod core;

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
    pub fn set_test_data(&mut self, value: i32) {
        self.core_.physics_data.data_blablabla = value;
    }
    pub fn get_test_data(&self) -> i32 {
        self.core_.physics_data.data_blablabla
    }
}