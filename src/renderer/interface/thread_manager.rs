pub mod core;
mod opengl_interface;
mod user_io;
mod texture;

/*
mod thread_manager:

manages the render thread.
*/

pub struct RenderThread {
    pub thread: std::thread::JoinHandle<()>
}

pub fn test_function() {
    for i in 1..1000 {
        println!("thread hi {}", i);
    }
}

impl RenderThread {
    pub fn new() -> Self {
        Self{thread: std::thread::spawn(|| {core::run().unwrap()})}
    }
}