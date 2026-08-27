pub mod renderer;
extern crate nalgebra as na;

/*
main:

uses physics::interface and renderer::interface to start the physics and renderer halves of the program.
*/

fn main() {
    
    //println!("Hello, world!");
    renderer::interface::start_thread();


    /*
    for i in 1..100 {
        println!("main hi {}", i);
    }

    let mut new_vector = na::Vector3::new(1.0, 0.0, 0.0);
    
    for _ in 1..100 {
        let rotation = na::UnitQuaternion::from_axis_angle(&na::Unit::new_normalize(na::Vector3::new(0.0, 1.0, 0.0)), 0.01);
        new_vector = rotation * new_vector;
        println!("{:?}", new_vector.transpose());
    }
    */
}