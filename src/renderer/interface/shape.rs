/*
mod shape:

defines structs Shape and ShapeManager for creating different rendered shapes.
*/

use crate::renderer::interface::core;
use std::collections::HashMap;

enum ShapeType {
    Polygon,
    Sphere,
    Torus,
    Cube,
}

// TODO: implement Eq and PartialEq
#[derive(Eq, PartialEq)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub radius: Option<f64>,
    pub radius_2: Option<f64>,
    pub vertex_count: Option<i64>,
}

impl Shape {
    pub fn new_polygon(radius: f64, vertex_count: i64) -> Shape {
        Shape {
            shape_type: ShapeType::Polygon,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_sphere(radius: f64, vertex_count: i64) -> Shape {
        Shape {
            shape_type: ShapeType::Sphere,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_torus(radius: f64, radius_2: f64, vertex_count: i64) -> Shape {
        Shape {
            shape_type: ShapeType::Torus,
            radius: Some(radius),
            radius_2: Some(radius_2),
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_cube(radius: f64) -> Shape {
        Shape {
            shape_type: ShapeType::Polygon,
            radius: Some(radius),
            radius_2: None,
            vertex_count: None,
        }
    }
}

struct ShapeManager {
    shapes: Vec<(Shape, Vec<core::Vertex>, Vec<u16>)>,
}

impl ShapeManager {
    pub fn get_shape_mesh(&mut self, shape: Shape) -> (Vec<core::Vertex>, Vec<u16>) {
        // use shapes like a hashmap but with no hashing. just check for the shape
        // in question using equality checks and add it if it doesnt exist
    }
}