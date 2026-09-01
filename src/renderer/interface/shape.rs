/*
mod shape:

defines structs Shape and ShapeManager for creating different rendered shapes.
*/

use crate::renderer::interface::core::{self, IndexedVertices};

#[derive(Eq, PartialEq, Clone, Copy)]
enum ShapeType {
    Polygon,
    Sphere,
    Torus,
    Cube,
}

// TODO: implement Eq and PartialEq
#[derive(Clone, Copy)]
pub struct Shape {
    shape_type: ShapeType,
    radius: Option<f32>,
    radius_2: Option<f32>,
    vertex_count: Option<u16>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        if (self.shape_type != other.shape_type) {return false;}
        if (self.radius != other.radius) {return false;}
        if (self.radius_2 != other.radius_2) {return false;}
        if (self.vertex_count != other.vertex_count) {return false;}
        return true;
    }
}

impl Eq for Shape {}

impl Shape {
    pub fn new_polygon(radius: f32, vertex_count: u16) -> Shape {
        Shape {
            shape_type: ShapeType::Polygon,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_sphere(radius: f32, vertex_count: u16) -> Shape {
        Shape {
            shape_type: ShapeType::Sphere,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_torus(radius: f32, radius_2: f32, vertex_count: u16) -> Shape {
        Shape {
            shape_type: ShapeType::Torus,
            radius: Some(radius),
            radius_2: Some(radius_2),
            vertex_count: Some(vertex_count),
        }
    }
    pub fn new_cube(radius: f32) -> Shape {
        Shape {
            shape_type: ShapeType::Cube,
            radius: Some(radius),
            radius_2: None,
            vertex_count: None,
        }
    }
    pub fn generate_indexed_vertices(self) -> core::IndexedVertices {
        if (self.shape_type == ShapeType::Polygon) {
            return self.generate_polygon();
        }
        todo!()
    }
    fn generate_polygon(self) -> core::IndexedVertices {
        let mut edge_vertices: Vec<core::Vertex> = Vec::new();
        for i in 0..self.vertex_count.unwrap() {
            let angle = std::f32::consts::TAU * (i as f32 / self.vertex_count.unwrap() as f32);
            edge_vertices.push(core::Vertex {
                position: [-angle.sin() * self.radius.unwrap(), angle.cos() * self.radius.unwrap(), 0.0],
                tex_coords: [(angle.sin() + 1.0) / 2.0, (-angle.cos() + 1.0) / 2.0]
            });
        }
        println!("{:?}", self.radius);
        let mut tri_vertices: Vec<core::Vertex> = Vec::new();
        for i in 0..(self.vertex_count.unwrap() - 2) {
            tri_vertices.push(edge_vertices[0]);
            tri_vertices.push(edge_vertices[(i+1) as usize]);
            tri_vertices.push(edge_vertices[(i+2) as usize]);
        }
        return IndexedVertices::from_vertices(tri_vertices);
    }
}

pub struct ShapeManager {
    shapes: Vec<(Shape, core::IndexedVertices)>,
}

impl ShapeManager {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new()
        }
    }
    pub fn get_shape_mesh(&mut self, shape: Shape) -> &core::IndexedVertices {
        // use shapes like a hashmap but with no hashing. just check for the shape
        // in question using equality checks and add it if it doesnt exist
        let index = self.shapes.iter().position(|s| s.0 == shape);
        match index {
            Some(i) => return &self.shapes[i].1,
            _ => {
                self.shapes.push((shape, shape.generate_indexed_vertices()));
                return &self.shapes.last().unwrap().1;
            },
        }
    }
}