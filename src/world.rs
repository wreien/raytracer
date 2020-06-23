//! The world to render.

use crate::geometry::Sphere;
use crate::utility::{ Colour, Vec3, Ray };
use crate::tracer::Tracer;

use image::{ RgbImage, Rgb };

/// The location and distribution of the camera view
#[derive(Debug)]
pub struct ViewPlane {
    pub hres: u32,
    /// horizontal resolution
    pub vres: u32,
    /// vertical resolution
    pub pixel_size: f64,
    pub gamma: f64,
}

/// The world itself
#[derive(Debug)]
pub struct World {
    pub background: Colour,
    pub sphere: Sphere,
    pub view: ViewPlane,
}

impl World {
    pub fn new() -> Self {
        Self {
            background: Colour::black(),
            sphere: Sphere {
                centre: Vec3::new(0.0, 0.0, 0.0),
                radius: 85.0,
            },
            view: ViewPlane {
                hres: 200,
                vres: 200,
                pixel_size: 1.0,
                gamma: 1.0,
            },
        }
    }

    pub fn render_scene<T: Tracer>(&self, tracer: T) -> RgbImage {
        let mut img = RgbImage::new(self.view.hres, self.view.vres);
        let zw = 100.0;
        let direction = Vec3::new(0.0, 0.0, -1.0);

        for row in 0..self.view.vres {
            for col in 0..self.view.hres {
                let c = col as f64;
                let r = row as f64;

                let x = self.view.pixel_size * (c - 0.5 * f64::from(self.view.hres - 1));
                let y = self.view.pixel_size * (r - 0.5 * f64::from(self.view.vres - 1));

                let origin = Vec3::new(x, y, zw);
                let ray = Ray { origin, direction };
                let colour = tracer.trace_ray(self, ray);
                img.put_pixel(row, col, Rgb::from(colour));
            }
        }

        img
    }
}
