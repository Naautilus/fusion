mod core;
mod state;
mod camera;
mod texture;
mod uniform;
mod descriptors;

/*
renderer::interface:

has functions that help start the renderer thread, and lets the renderer thread access physics data.
*/

pub struct Interface {
    pub thread: std::thread::JoinHandle<()>
}

impl Interface {
    pub fn start_thread() -> Self {
        Self{thread: std::thread::spawn(|| {core::run().unwrap()})}
    }
}