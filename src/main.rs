// bulk of renderer code from https://sotrh.github.io/learn-wgpu/beginner/

mod renderer;

fn main() {

    let _renderer_interface = renderer::interface::Interface::start_thread();
    loop {}
}