mod geometry;
mod tracer;
mod utility;
mod world;

fn main() {
    let filename = std::env::args().nth(1).unwrap_or("demo.png".to_string());
    let world = world::World::new();
    let tracer = tracer::MultipleObjectTracer {};
    let scene = world.render_scene(tracer);
    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}
