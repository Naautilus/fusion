pub mod thread_manager;

/*
renderer::interface:

has functions that help start the renderer thread, and lets the renderer thread access physics data.
*/

pub fn start_thread() {
    thread_manager::RenderThread::new();
}