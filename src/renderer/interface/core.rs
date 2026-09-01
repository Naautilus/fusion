/*
mod core:

the core of the renderer; controls the logic of object positioning,
and uses user_io and opengl_interface to abstract away the opengl/io layer.
*/

use std::collections::btree_map::Range;

use crate::renderer::interface::state;

use winit::{
    event_loop::EventLoop, platform::x11::EventLoopBuilderExtX11
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

#[derive(Clone)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
        }
    }
}

#[derive(Clone)]
pub struct IndexedVertices {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl IndexedVertices {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        Self {
            vertices: vertices.clone(),
            indices: (0..vertices.len() as u16).collect::<Vec<u16>>(),
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    {
        env_logger::init();
    }

    let mut event_loop_builder = EventLoop::with_user_event();
    EventLoopBuilderExtX11::with_any_thread(&mut event_loop_builder, true);
    let event_loop = event_loop_builder.build()?;
    {
        let mut app = state::App::new();
        event_loop.run_app(&mut app)?;
    }

    Ok(())
}
