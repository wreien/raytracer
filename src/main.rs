use raytracer::{
    camera::{self, Camera},
    geometry::{self, Geometry},
    light::{self, Light},
    material::Matte,
    sampler::Default as Sampler,
    tracer::MultipleObjectTracer,
    utility::{Colour, Vec3},
    world::{ViewPlane, World},
};

use std::env;
use std::time::Instant;

fn main() {
    let filename = env::args().nth(1).unwrap_or("demo.png".to_string());

    let now = Instant::now();

    let (world, camera) = build_scene();
    let scene = camera.render_scene(&world, MultipleObjectTracer {});

    let elapsed = now.elapsed().as_millis();
    println!("Rendered in {} seconds.", elapsed as f64 / 1000.0);

    match scene.save(&filename) {
        Ok(_) => println!("Saved to \"{}\".", filename),
        Err(_) => println!("Failed to write to \"{}\".", filename),
    }
}

fn build_scene() -> (World, impl Camera) {
    let sampler = Sampler::new(25);
    let view = ViewPlane::new(400, 300, 0.05, sampler);

    let location = camera::Location {
        eye: Vec3::new(-10.0, 5.0, 50.0),
        centre: Vec3::new(0.0, 5.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
    };
    let view_len = 40.0;
    let camera = camera::Pinhole::new(location, view_len, 0.8);

    let ambient = Box::new(light::Ambient::new(1.0));
    let mut lights: Vec<Box<dyn Light>> = Vec::new();
    lights.push(Box::new(light::PointLight::new(
        2.0,
        Vec3::new(-50.0, 50.0, 0.0),
    )));

    let mut objects: Vec<Box<dyn Geometry>> = vec![];
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(7.0, 4.0, 3.0),
        radius: 4.0,
        material: Matte::new(0.5, 1.0, Colour::red()),
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(0.0, 4.0, -24.0),
        radius: 4.0,
        material: Matte::new(0.5, 1.0, Colour::new(1.0, 1.0, 0.0)),
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(-7.0, 4.0, -51.0),
        radius: 4.0,
        material: Matte::new(0.5, 1.0, Colour::blue()),
    }));
    objects.push(Box::new(geometry::Sphere {
        centre: Vec3::new(-14.0, 4.0, -78.0),
        radius: 4.0,
        material: Matte::new(0.5, 1.0, Colour::white()),
    }));
    objects.push(Box::new(geometry::Plane {
        point: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Matte::new(0.8, 0.4, Colour::new(0.0, 0.3, 0.0)),
    }));
    objects.push(Box::new(geometry::Cuboid {
        min: Vec3::new(40.0, 0.0, -130.0),
        max: Vec3::new(10.0, 15.0, -80.0),
        material: Matte::new(0.5, 1.0, Colour::green()),
    }));

    (
        World {
            objects,
            background: Colour::new(0.7, 0.7, 1.0),
            view,
            ambient,
            lights,
        },
        camera,
    )
}
