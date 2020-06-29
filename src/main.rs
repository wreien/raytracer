mod camera;
mod geometry;
mod sampler;
mod tracer;
mod utility;
mod world;

use camera::{Camera, Location};
use std::env;
use utility::Vec3;
use world::World;

fn main() {
    let filename = env::args().nth(1).unwrap_or("demo.png".to_string());

    let sampler = sampler::Default::new(256);
    let world = World::new(Box::new(sampler.clone()));

    let location = Location {
        eye: Vec3::new(0.0, 5.0, 50.0),
        centre: Vec3::new(0.0, 5.0, 0.0),
        up: utility::Vec3::new(0.0, 1.0, 0.0),
    };
    let view_len = 40.0;
    let focal_len = 74.0;

    //let camera = ThinLens::new(location, view_len, focal_len, 1.0, 1.0, sampler);
    let camera = camera::Pinhole::new(location, view_len, 1.0);

    let tracer = tracer::MultipleObjectTracer {};
    let scene = camera.render_scene(&world, tracer);
    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}
