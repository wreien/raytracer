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
    pub fn new(sampler: Box<dyn sampler::Generator>) -> Self {
        let sphere_1 = Box::new(geometry::Sphere {
            centre: Vec3::new(7.0, 4.0, 3.0),
            radius: 4.0,
            colour: Colour::red(),
        });
        let sphere_2 = Box::new(geometry::Sphere {
            centre: Vec3::new(0.0, 4.0, -24.0),
            radius: 4.0,
            colour: Colour::new(1.0, 1.0, 0.0), // yellow
        });
        let sphere_3 = Box::new(geometry::Sphere {
            centre: Vec3::new(-7.0, 4.0, -51.0),
            radius: 4.0,
            colour: Colour::blue(),
        });
        let sphere_4 = Box::new(geometry::Sphere {
            centre: Vec3::new(-21.0, 4.0, -99.0),
            radius: 4.0,
            colour: Colour::white(),
        });
        let plane = Box::new(geometry::Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            colour: Colour::new(0.0, 0.3, 0.0), // dark green
        });

        Self {
            background: Colour::black(),
            objects: vec![sphere_1, sphere_2, sphere_3, sphere_4, plane],
            view: ViewPlane {
                hres: 400,
                vres: 300,
                s: 0.05,
                gamma: 1.0,
                sampler
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
