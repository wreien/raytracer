//! The world to render.

use crate::geometry::{self, Geometry, Intersection};
use crate::sampler::{Generator, Jittered};
use crate::tracer::Tracer;
use crate::utility::{Colour, Ray, Vec3};

use image::{Rgb, RgbImage};

/// General information about the view.
///
/// Includes things like the position and scale of viewing, and other
/// information relevant to viewing like the number of samples to use for
/// antialiasing.
#[derive(Debug)]
pub struct ViewPlane {
    /// The horizontal resolution of the result image.
    pub hres: u32,
    /// The vertical resolution of the result image.
    pub vres: u32,
    /// The size of a pixel in the image; the scaling factor.
    pub s: f64,
    /// Gamma correction to apply. (Currently unused.)
    pub gamma: f64,
    /// Sampler for antialiasing
    pub sampler: Box<dyn Generator>,
}

/// The world itself.
#[derive(Debug)]
pub struct World {
    pub background: Colour,
    pub objects: Vec<Box<dyn Geometry>>,
    pub view: ViewPlane,
}

impl World {
    /// Builds the world.
    ///
    /// Currently this is the place to change the scenery that is displayed.
    pub fn new() -> Self {
        let sphere_1 = Box::new(geometry::Sphere {
            centre: Vec3::new(0.0, -25.0, 0.0),
            radius: 80.0,
            colour: Colour::red(),
        });
        let sphere_2 = Box::new(geometry::Sphere {
            centre: Vec3::new(0.0, 30.0, 0.0),
            radius: 60.0,
            colour: Colour::new(1.0, 1.0, 0.0), // yellow
        });
        let plane = Box::new(geometry::Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 1.0).normalise(),
            colour: Colour::new(0.0, 0.3, 0.0), // dark green
        });

        Self {
            background: Colour::black(),
            objects: vec![sphere_1, sphere_2, plane],
            view: ViewPlane {
                hres: 200,
                vres: 200,
                s: 1.0,
                gamma: 1.0,
                sampler: Box::new(Jittered::new(25)),
            },
        }
    }

    /// Returns the intersection for the first object hit by the given ray.
    pub fn hit_objects(&self, ray: Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(&ray))
            .min_by(|a, b| a.t.partial_cmp(&b.t).expect("distance is NaN"))
    }

    /// Renders the scene.
    ///
    /// You may pass a different tracer to render in different ways.
    /// See the [`tracer`][crate::tracer] module for more details.
    pub fn render_scene<T: Tracer>(&self, tracer: T) -> RgbImage {
        let mut img = RgbImage::new(self.view.hres, self.view.vres);
        let zw = 100.0;
        let direction = Vec3::new(0.0, 0.0, -1.0);

        let mut samples = self.view.sampler.gen_square_samples();

        let width = f64::from(self.view.hres - 1);
        let height = f64::from(self.view.vres - 1);

        for col in 0..self.view.hres {
            for row in 0..self.view.vres {
                let c = col as f64;
                let r = row as f64;

                #[rustfmt::skip]
                let colour = samples
                    .get_next()
                    .iter()
                    .fold(Colour::black(), |accum, sample| {
                        let x = self.view.s * (width / 2.0 - c + sample.x);
                        let y = self.view.s * (height / 2.0 - r + sample.y);

                        let origin = Vec3::new(x, y, zw);
                        let ray = Ray { origin, direction };
                        let colour = tracer.trace_ray(self, ray);

                        accum + colour
                    });
                let colour = colour / (samples.num_samples() as f64);

                img.put_pixel(col, row, Rgb::from(colour));
            }
        }

        img
    }
}
