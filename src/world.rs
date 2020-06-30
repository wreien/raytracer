//! The world to render.

use crate::geometry::{Geometry, Intersection};
use crate::sampler;
use crate::utility::{Colour, Ray};

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

impl ViewPlane {
    pub fn new<S>(hres: u32, vres: u32, s: f64, sampler: S) -> Self
    where
        S: sampler::Generator + 'static,
    {
        Self {
            hres,
            vres,
            s,
            gamma: 1.0,
            sampler: Box::new(sampler),
        }
    }
}

/// The world itself.
#[derive(Debug)]
pub struct World {
    pub background: Colour,
    pub objects: Vec<Box<dyn Geometry>>,
    pub view: ViewPlane,
}

impl World {
    /// Create the world.
    pub fn new(
        objects: Vec<Box<dyn Geometry>>,
        view: ViewPlane,
        background: Colour,
    ) -> Self {
        Self {
            background,
            objects,
            view,
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
