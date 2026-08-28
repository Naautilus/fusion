pub mod thread_manager;

/*
renderer::interface:

has functions that help start the renderer thread, and lets the renderer thread access physics data.
*/



pub struct Interface {
    render_thread: thread_manager::RenderThread,
}

impl Interface {
    pub fn start_thread() -> Self {
        Self{render_thread: thread_manager::RenderThread::new()}
    }
}