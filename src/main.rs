use raytracer::{
    camera::{self, Camera},
    geometry::{self, Geometry},
    sampler::{self, Default as Sampler},
    tracer::MultipleObjectTracer,
    utility::{Colour, Vec3},
    world::{ViewPlane, World},
};

use std::env;
use std::time::Instant;

fn main() {
    let filename = env::args().nth(1).unwrap_or("demo.png".to_string());

    let now = Instant::now();

    let sampler = Sampler::new(256);
    let camera = setup_camera(sampler.clone());
    let viewplane = ViewPlane::new(400, 300, 0.05, sampler);

    let objects = build_scene();
    let world = World::new(objects, viewplane);
    let scene = camera.render_scene(&world, MultipleObjectTracer {});

    let elapsed = now.elapsed().as_millis();
    println!("Rendered in {} seconds.", elapsed as f64 / 1000.0);

    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}

fn setup_camera<S: sampler::Generator>(sampler: S) -> impl Camera {
    let location = camera::Location {
        eye: Vec3::new(-10.0, 5.0, 50.0),
        centre: Vec3::new(0.0, 5.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
    };
    let view_len = 40.0;

    //---

    let focal_len = 74.0;
    camera::ThinLens::new(location, view_len, focal_len, 1.0, 1.0, sampler)

    // camera::Pinhole::new(location, view_len, 1.0)
}

fn build_scene() -> Vec<Box<dyn Geometry>> {
    let mut objects: Vec<Box<dyn Geometry>> = vec![];

    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(7.0, 4.0, 3.0),
        radius: 4.0,
        colour: Colour::red(),
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(0.0, 4.0, -24.0),
        radius: 4.0,
        colour: Colour::new(1.0, 1.0, 0.0), // yellow
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(-7.0, 4.0, -51.0),
        radius: 4.0,
        colour: Colour::blue(),
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(-14.0, 4.0, -78.0),
        radius: 4.0,
        colour: Colour::white(),
    }));

    objects.push(Box::new(geometry::Plane {
        point: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        colour: Colour::new(0.0, 0.3, 0.0), // dark green
    }));

    objects.push(Box::new(geometry::Cuboid {
        min: Vec3::new(40.0, 0.0, -130.0),
        max: Vec3::new(10.0, 15.0, -80.0),
        colour: Colour::green(),
    }));

    objects
}
