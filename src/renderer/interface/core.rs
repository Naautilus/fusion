/*
mod core:

the core of the renderer; controls the logic of object positioning,
and uses user_io and opengl_interface to abstract away the opengl/io layer.
*/

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



pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], },
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], },
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], },
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], },
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], },
];

pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];


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
