#![allow(dead_code)]

use std::fmt::Debug;

use crate::tracer::Tracer;
use crate::utility::{Colour, Ray, Vec2, Vec3};
use crate::world::World;

use image::{Rgb, RgbImage};

/// Renders scenes.
///
/// Different cameras will use different projections and/or techniques to do rendering.
/// Look at the documentation for each individual camera to determine what they do.
pub trait Camera: Debug {
    /// Renders the scene for the given `world`.
    ///
    /// You may pass different tracers to render in different ways.
    /// See the [`tracer`][crate::tracer] module for more details.
    ///
    /// Returns the rendered image buffer.
    fn render_scene<T: Tracer>(&self, world: &World, tracer: T) -> RgbImage;
}

fn compute_basis_vectors(eye: Vec3, centre: Vec3, up: Vec3) -> (Vec3, Vec3, Vec3) {
    let w = (eye - centre).normalise();
    let u = up.cross(w).normalise();
    let v = w.cross(u);
    (u, v, w)
}

/// A virtual pinhole camera.
///
/// This is a perspective camera with arbitrary eye points, view directions,
/// orientation, and distance to the view plane. Although simple, it's probably
/// a decent default for now.
#[derive(Debug)]
pub struct Pinhole {
    /// Ratio of exposure.
    pub exposure: f64,
    /// Distance to the view plane.
    pub distance: f64,
    /// Zoom factor.
    pub zoom: f64,

    /// The position of the camera.
    eye: Vec3,
    /// Where the camera is facing.
    centre: Vec3,
    /// The "up" direction of the camera.
    up: Vec3,

    /// Orthonormal basis vectors for the camera.
    basis: (Vec3, Vec3, Vec3),
}

impl Pinhole {
    pub fn new(eye: Vec3, centre: Vec3, up: Vec3, distance: f64, zoom: f64) -> Self {
        let basis = compute_basis_vectors(eye, centre, up);
        Self {
            eye,
            centre,
            up,
            exposure: 1.0,
            basis,
            distance,
            zoom,
        }
    }

    fn ray_direction(&self, p: Vec2) -> Vec3 {
        let (u, v, w) = self.basis;
        (u * p.x + v * p.y - w * self.distance).normalise()
    }
}

impl Camera for Pinhole {
    fn render_scene<T: Tracer>(&self, world: &World, tracer: T) -> RgbImage {
        let mut img = RgbImage::new(world.view.hres, world.view.vres);
        let mut samples = world.view.sampler.gen_square_samples();
        let num_samples = samples.num_samples() as f64;

        let width = f64::from(world.view.hres - 1);
        let height = f64::from(world.view.vres - 1);

        let scale = world.view.s / self.zoom;
        let origin = self.eye;

        for col in 0..world.view.hres {
            for row in 0..world.view.vres {
                let c = col as f64;
                let r = row as f64;
                let pixel = Vec2::new(width * 0.5 - c, height * 0.5 - r);

                #[rustfmt::skip]
                let colour = samples
                    .get_next()
                    .iter()
                    .fold(Colour::black(), |accum, &sample| {
                        let point = (pixel + sample) * scale;
                        let direction = self.ray_direction(point);

                        let ray = Ray { origin, direction };
                        let colour = tracer.trace_ray(world, ray);

                        accum + colour
                    });
                let colour = colour * self.exposure / num_samples;

                img.put_pixel(col, row, Rgb::from(colour));
            }
        }

        return img;
    }
}
