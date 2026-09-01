/*
mod camera:

structs Camera and CameraController are defined here.
*/

use cgmath::Vector3;
use nalgebra::{self as na, Unit};

use winit::{
    keyboard::{KeyCode}
};

pub struct Camera {
    pub eye: na::Point3<f32>,
    pub target: na::Point3<f32>,
    pub up: na::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: na::Matrix4<f32> = na::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0);

impl Camera {
    pub fn build_view_projection_matrix(&self) -> na::Matrix4<f32> {
        let view = na::Matrix4::look_at_rh(&self.eye, &self.target, &self.up);

        let proj = na::Matrix4::new_perspective(self.aspect, self.fovy, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

pub struct CameraController {
    translate_speed: f32,
    rotate_speed: f32,
    translation: na::Vector3<f32>,
    rotation: na::Vector3<f32>,
}

impl CameraController {
    pub fn new(translate_speed: f32, rotate_speed: f32) -> Self {
        Self {
            translate_speed: translate_speed,
            rotate_speed: rotate_speed,
            translation: na::Vector3::new(0.0, 0.0, 0.0),
            rotation: na::Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) {
        match code {
            KeyCode::KeyD => {self.translation.x = if is_pressed { 1.0} else {0.0};}
            KeyCode::KeyA => {self.translation.x = if is_pressed {-1.0} else {0.0};}
            KeyCode::KeyE => {self.translation.y = if is_pressed { 1.0} else {0.0};}
            KeyCode::KeyQ => {self.translation.y = if is_pressed {-1.0} else {0.0};}
            KeyCode::KeyW => {self.translation.z = if is_pressed { 1.0} else {0.0};}
            KeyCode::KeyS => {self.translation.z = if is_pressed {-1.0} else {0.0};}

            KeyCode::ArrowUp    => {self.rotation.x = if is_pressed { 1.0} else {0.0};}
            KeyCode::ArrowDown  => {self.rotation.x = if is_pressed {-1.0} else {0.0};}
            KeyCode::ArrowLeft  => {self.rotation.y = if is_pressed { 1.0} else {0.0};}
            KeyCode::ArrowRight => {self.rotation.y = if is_pressed {-1.0} else {0.0};}

            _ => {}
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {

        let mut position = camera.eye;
        let forward = (camera.target - camera.eye).normalize();
        let right = forward.cross(&camera.up);

        position += right * self.translate_speed * self.translation.x;
        position += right.cross(&forward) * self.translate_speed * self.translation.y;
        position += forward * self.translate_speed * self.translation.z;

        let rotation = na::Rotation3::from_axis_angle(
            &na::UnitVector3::new_normalize(right), 
            self.rotation.x * self.rotate_speed, 
        );
        let forward = rotation * forward;

        let rotation = na::Rotation3::from_axis_angle(
            &na::UnitVector3::new_normalize(camera.up), 
            self.rotation.y * self.rotate_speed, 
        );
        let mut forward = rotation * forward;
        forward.y = forward.y.clamp(-0.9, 0.9);
        forward = forward.normalize();

        camera.eye = position;
        camera.target = camera.eye + forward;
        camera.up = na::Vector3::new(0.0, 1.0, 0.0);

        
    }
}