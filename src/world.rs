//! The world to render.

use crate::geometry::{self, Geometry, Intersection};
use crate::sampler;
use crate::utility::{Colour, Ray, Vec3};

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
    pub sampler: Box<dyn sampler::Generator>,
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
            centre: Vec3::new(40.0, 80.0, 50.0),
            radius: 80.0,
            colour: Colour::red(),
        });
        let sphere_2 = Box::new(geometry::Sphere {
            centre: Vec3::new(-50.0, 60.0, 0.0),
            radius: 60.0,
            colour: Colour::new(1.0, 1.0, 0.0), // yellow
        });
        let plane = Box::new(geometry::Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
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
                sampler: Box::new(sampler::Default::new(25)),
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
}
