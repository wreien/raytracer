#![allow(dead_code)]

use std::fmt::Debug;

use crate::sampler::Generator;
use crate::tracer::Tracer;
use crate::utility::{Colour, Ray, Vec2, Vec3};
use crate::world::World;

use image::{Rgb, RgbImage};
use indicatif::ProgressIterator;

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

/// A user-specified camera location.
#[derive(Debug, Clone)]
pub struct Location {
    /// The position of the camera.
    pub eye: Vec3,
    /// Where the camera is looking.
    pub centre: Vec3,
    /// What direction is "up" for the camera.
    pub up: Vec3,
}

/// Given a location in camera coords, calculate the orthonormal basis vectors.
///
/// Will panic if `up` and `eye - centre` are parallel.
fn compute_basis_vectors(Location { eye, centre, up }: &Location) -> (Vec3, Vec3, Vec3) {
    let w = (*eye - *centre).normalise();
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
    pub view_len: f64,
    /// Zoom factor.
    pub zoom: f64,

    /// The position of the camera.
    eye: Vec3,
    /// Orthonormal basis vectors for the camera.
    basis: (Vec3, Vec3, Vec3),
}

impl Pinhole {
    pub fn new(location: Location, view_len: f64, zoom: f64) -> Self {
        let basis = compute_basis_vectors(&location);
        Self {
            eye: location.eye,
            exposure: 1.0,
            basis,
            view_len,
            zoom,
        }
    }

    fn ray_direction(&self, p: Vec2) -> Vec3 {
        let (u, v, w) = self.basis;
        (u * p.x + v * p.y - w * self.view_len).normalise()
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

        for col in (0..world.view.hres).progress() {
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

/// Camera with depth-of-field simulation.
///
/// This approximates a camera with a thin lens of finite width, in comparison to
/// the infinitely small lens of the [`Pinhole`] camera. This can be used to approximate
/// depth-of-field, where the focal plane of the scenery is in focus and objects further
/// away from the focal plane become progressively less in-focus.
#[derive(Debug)]
pub struct ThinLens<G: Generator> {
    /// Ratio of exposure.
    pub exposure: f64,
    /// Size of the lens
    pub lens_radius: f64,
    /// View plane distance
    pub view_len: f64,
    /// Focal plane distance
    pub focal_len: f64,
    /// Zoom factor
    pub zoom: f64,
    /// Sampler used to generate rays from the lens
    pub sampler: G,

    /// The position of the camera.
    eye: Vec3,
    /// Orthonormal basis vectors for the camera.
    basis: (Vec3, Vec3, Vec3),
}

impl<G: Generator> ThinLens<G> {
    pub fn new(
        location: Location,
        view_len: f64,
        focal_len: f64,
        lens_radius: f64,
        zoom: f64,
        sampler: G,
    ) -> Self {
        let basis = compute_basis_vectors(&location);
        Self {
            exposure: 1.0,
            lens_radius,
            view_len,
            focal_len,
            zoom,
            sampler,
            eye: location.eye,
            basis,
        }
    }

    fn ray_direction(&self, pixel_point: Vec2, lens_point: Vec2) -> Vec3 {
        let hit_point = pixel_point * self.focal_len / self.view_len;
        let offset = hit_point - lens_point;
        let (u, v, w) = self.basis;
        (offset.x * u + offset.y * v - self.focal_len * w).normalise()
    }

    fn ray_origin(&self, lens_point: Vec2) -> Vec3 {
        let (u, v, _) = self.basis;
        self.eye + lens_point.x * u + lens_point.y * v
    }
}

impl<G: Generator> Camera for ThinLens<G> {
    fn render_scene<T: Tracer>(&self, world: &World, tracer: T) -> RgbImage {
        let mut img = RgbImage::new(world.view.hres, world.view.vres);
        let mut antialias = world.view.sampler.gen_square_samples();
        let mut disk_pnt = self.sampler.gen_disc_samples();

        assert!(antialias.num_samples() == disk_pnt.num_samples());
        let num_samples = antialias.num_samples() as f64;

        let width = f64::from(world.view.hres - 1);
        let height = f64::from(world.view.vres - 1);

        let scale = world.view.s / self.zoom;

        for col in (0..world.view.hres).progress() {
            for row in 0..world.view.vres {
                let c = col as f64;
                let r = row as f64;
                let pixel = Vec2::new(width * 0.5 - c, height * 0.5 - r);

                let colour = antialias
                    .get_next()
                    .iter()
                    .zip(disk_pnt.get_next().iter())
                    .fold(Colour::black(), |accum, (&sample, &disc_point)| {
                        let pixel_point = (pixel + sample) * scale;
                        let lens_point = disc_point * self.lens_radius;

                        let ray = Ray {
                            origin: self.ray_origin(lens_point),
                            direction: self.ray_direction(pixel_point, lens_point),
                        };
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
