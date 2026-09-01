/*
mod shape:

defines structs Shape and ShapeManager for creating different rendered shapes.
*/

use crate::renderer::interface::core::{self, IndexedVertices, Vertex};
use nalgebra as na;
use nalgebra::Vector3 as Vec3;
use nalgebra::Rotation3 as Rot3;

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
    vertex_count: Option<u32>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        if self.shape_type != other.shape_type {return false;}
        if self.radius != other.radius {return false;}
        if self.radius_2 != other.radius_2 {return false;}
        if self.vertex_count != other.vertex_count {return false;}
        return true;
    }
}

impl Eq for Shape {}

impl Shape {
    pub fn new_polygon(radius: f32, vertex_count: u32) -> Shape {
        Shape {
            shape_type: ShapeType::Polygon,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }

    pub fn new_sphere(radius: f32, vertex_count: u32) -> Shape {
        Shape {
            shape_type: ShapeType::Sphere,
            radius: Some(radius),
            radius_2: None,
            vertex_count: Some(vertex_count),
        }
    }

    pub fn new_torus(radius: f32, radius_2: f32, vertex_count: u32) -> Shape {
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

    pub fn generate_indexed_vertices(self) -> IndexedVertices {
        match (self.shape_type) {
            ShapeType::Polygon => {return self.generate_polygon();}
            ShapeType::Sphere => {return self.generate_sphere();}
            ShapeType::Torus => {return self.generate_torus();}
            ShapeType::Cube => {return self.generate_cube();}
            _ => todo!()
        }
    }

    fn generate_polygon(self) -> IndexedVertices {
        let mut edge_vertices: Vec<Vertex> = Vec::new();
        for i in 0..self.vertex_count.unwrap() {
            let angle = std::f32::consts::TAU * (i as f32 / self.vertex_count.unwrap() as f32);
            edge_vertices.push(Vertex {
                position: [-angle.sin() * self.radius.unwrap(), angle.cos() * self.radius.unwrap(), 0.0],
                tex_coords: [(angle.sin() + 1.0) / 2.0, (-angle.cos() + 1.0) / 2.0]
            });
        }
        //println!("{:?}", self.radius);
        let mut tri_vertices: Vec<Vertex> = Vec::new();
        for i in 0..(self.vertex_count.unwrap() - 2) {
            tri_vertices.push(edge_vertices[0]);
            tri_vertices.push(edge_vertices[(i+1) as usize]);
            tri_vertices.push(edge_vertices[(i+2) as usize]);
        }
        return IndexedVertices::from_vertices(tri_vertices);
    }

    fn generate_sphere(self) -> IndexedVertices {
        let mut tri_vertices: Vec<Vertex> = Vec::new();
        for x in 0..self.vertex_count.unwrap() {
            for y in 0..self.vertex_count.unwrap()/2 {
                fn vertex(azimuth: f32, elevation: f32, radius: f32) -> Vertex {
                    return Vertex {
                        position: [azimuth.cos() * elevation.cos(), -elevation.sin(), azimuth.sin() * elevation.cos()].map(|n| n * radius),
                        tex_coords: [1.0-(azimuth / std::f32::consts::TAU), (elevation/std::f32::consts::TAU+0.5)],
                    }
                }
                let tau = std::f32::consts::TAU;
                let pi = std::f32::consts::PI;
                let azimuth_0 = tau * (x as f32 / self.vertex_count.unwrap() as f32);
                let azimuth_1 = tau * ((x+1) as f32 / self.vertex_count.unwrap() as f32);
                let elevation_0 = -pi/2.0 + pi * (y as f32 / (self.vertex_count.unwrap()/2) as f32);
                let elevation_1 = -pi/2.0 + pi * ((y+1) as f32 / (self.vertex_count.unwrap()/2) as f32);

                tri_vertices.push(vertex(azimuth_0, elevation_0, self.radius.unwrap()));
                tri_vertices.push(vertex(azimuth_1, elevation_0, self.radius.unwrap()));
                tri_vertices.push(vertex(azimuth_0, elevation_1, self.radius.unwrap()));

                tri_vertices.push(vertex(azimuth_1, elevation_1, self.radius.unwrap()));
                tri_vertices.push(vertex(azimuth_0, elevation_1, self.radius.unwrap()));
                tri_vertices.push(vertex(azimuth_1, elevation_0, self.radius.unwrap()));
            }
        }
        return IndexedVertices::from_vertices(tri_vertices);
    }

    fn generate_torus(self) -> IndexedVertices {
        let mut tri_vertices: Vec<Vertex> = Vec::new();
        for x in 0..self.vertex_count.unwrap() {
            for y in 0..self.vertex_count.unwrap() {
                fn vertex(azimuth: f32, elevation: f32, radius: f32, radius_2: f32) -> Vertex {
                    let mut position: Vec3<_> = Vec3::new(radius + elevation.cos()*radius_2, elevation.sin()*radius_2, 0.0);
                    position = Rot3::new(Vec3::new(0.0, azimuth, 0.0)).matrix() * position;
                    return Vertex {
                        position: [position.x, position.y, position.z],
                        tex_coords: [1.0-(azimuth / std::f32::consts::TAU), 1.0-(elevation/std::f32::consts::TAU+0.5)],
                    }
                }
                let tau = std::f32::consts::TAU;
                let pi = std::f32::consts::PI;
                let azimuth_0 = tau * (x as f32 / self.vertex_count.unwrap() as f32);
                let azimuth_1 = tau * ((x+1) as f32 / self.vertex_count.unwrap() as f32);
                let elevation_0 = -pi + tau * (y as f32 / (self.vertex_count.unwrap()) as f32);
                let elevation_1 = -pi + tau * ((y+1) as f32 / (self.vertex_count.unwrap()) as f32);

                tri_vertices.push(vertex(azimuth_0, elevation_0, self.radius.unwrap(), self.radius_2.unwrap()));
                tri_vertices.push(vertex(azimuth_1, elevation_0, self.radius.unwrap(), self.radius_2.unwrap()));
                tri_vertices.push(vertex(azimuth_0, elevation_1, self.radius.unwrap(), self.radius_2.unwrap()));

                tri_vertices.push(vertex(azimuth_1, elevation_1, self.radius.unwrap(), self.radius_2.unwrap()));
                tri_vertices.push(vertex(azimuth_0, elevation_1, self.radius.unwrap(), self.radius_2.unwrap()));
                tri_vertices.push(vertex(azimuth_1, elevation_0, self.radius.unwrap(), self.radius_2.unwrap()));
            }
        }
        return IndexedVertices::from_vertices(tri_vertices);
    }

    fn generate_cube(self) -> IndexedVertices {
        let quad_vertices: Vec<_> = vec![
            // (uv is the typical cubemap format, like assembling a minecraft paper cube, with bottom centered, +x to right, +z up)
            // (in this case, the uv numbers are *4 to be easier to enter, since the uv is squares on a 4x4 grid)
            //x,y,z   u,v
            // -y side (bottom)
            ([0,0,0],[1,3]),
            ([0,0,1],[1,2]),
            ([1,0,1],[2,2]), // old z- (now lower square)
            ([1,0,0],[2,3]),
            // +y side (top)
            ([0,1,0],[1,0]),
            ([1,1,0],[2,0]), // old z+ (now upper square)
            ([1,1,1],[2,1]),
            ([0,1,1],[1,1]),
            // -x side (left)
            ([0,0,0],[0,2]),
            ([0,1,0],[0,1]),
            ([0,1,1],[1,1]),
            ([0,0,1],[1,2]),
            // +x side (right)
            ([1,0,0],[3,2]),
            ([1,0,1],[2,2]),
            ([1,1,1],[2,1]),
            ([1,1,0],[3,1]),
            // -z side (back)
            ([0,0,0],[4,2]),
            ([1,0,0],[3,2]), // old bottom (now center square)
            ([1,1,0],[3,1]),
            ([0,1,0],[4,1]),
            // +z side (front)
            ([0,0,1],[1,2]),
            ([0,1,1],[1,1]), // old top (now opposite square)
            ([1,1,1],[2,1]),
            ([1,0,1],[2,2]),
        ];
        let mut tri_vertices = Vec::new();
        let mut i = 0;
        while i < quad_vertices.len() {
            tri_vertices.push(quad_vertices[i+0]);
            tri_vertices.push(quad_vertices[i+3]);
            tri_vertices.push(quad_vertices[i+1]);

            tri_vertices.push(quad_vertices[i+2]);
            tri_vertices.push(quad_vertices[i+1]);
            tri_vertices.push(quad_vertices[i+3]);
            i += 4;
        }
        return IndexedVertices::from_vertices(
            tri_vertices
            .iter()
            .map(|v| Vertex{
                position: v.0.map(|n| ((n as f32)*2.0-1.0) * self.radius.unwrap()),
                tex_coords: [v.1[0] as f32/4.0, v.1[1] as f32/3.0]
            }).collect()
        );
    }

}

pub struct ShapeManager {
    shapes: Vec<(Shape, IndexedVertices)>,
}

impl ShapeManager {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new()
        }
    }
    pub fn get_shape_mesh(&mut self, shape: Shape) -> &IndexedVertices {
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