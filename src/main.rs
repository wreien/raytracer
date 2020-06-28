mod camera;
mod geometry;
mod sampler;
mod tracer;
mod utility;
mod world;

use std::env;
use world::World;
use utility::Vec3;
use camera::{Camera, Pinhole};

fn main() {
    let filename = env::args().nth(1).unwrap_or("demo.png".to_string());
    let world = World::new();
    let camera;
    {
        let eye = Vec3::new(0.0, 100.0, 500.0);
        let centre = Vec3::new(0.0, 50.0, 0.0);
        let up = utility::Vec3::new(0.0, 1.0, 0.0);
        let distance = 300.0;
        let zoom = 1.0;

        camera = Pinhole::new(eye, centre, up, distance, zoom);
    }

    let tracer = tracer::MultipleObjectTracer {};
    let scene = camera.render_scene(&world, tracer);
    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}
