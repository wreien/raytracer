mod geometry;
mod tracer;
mod utility;
mod world;

use tracer::SimpleTracer;
use world::World;

use std::env;

fn main() {
    let filename = env::args().nth(1).unwrap_or("demo.png".to_string());
    let world = World::new();
    let tracer = SimpleTracer {};
    let scene = world.render_scene(tracer);
    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}
