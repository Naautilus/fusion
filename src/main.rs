// bulk of renderer code from https://sotrh.github.io/learn-wgpu/beginner/

mod renderer;
mod physics;

fn main() {

    let renderer_interface = renderer::interface::Interface::start_thread();
    let mut physics_interface = physics::interface::Interface::initialize_physics_engine();
    physics_interface.run_physics_loop();
    loop {}
}