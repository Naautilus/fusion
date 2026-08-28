// bulk of renderer code from https://sotrh.github.io/learn-wgpu/beginner/

use core::time;

mod renderer;

fn main() {

    let renderer_interface = renderer::interface::Interface::start_thread();
    loop {}
}