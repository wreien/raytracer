//! Materials.
//!
//! These are used to shade objects in different manners: by applying different
//! materials you can get an objects behaviour in regards to light to change.
//!
//! See the currently available [BRDFs](crate::brdf) for reflection functions
//! used in materials.

use crate::brdf::{Lambertian, BRDF};
use crate::utility::Colour;
use crate::world::Intersection;

use std::fmt::Debug;

/// A material that can be applied to an object.
pub trait Material: Debug {
    /// Returns the output colour of the point at the given intersection point.
    fn shade(&self, hit: &Intersection) -> Colour;
}

/// Matte objects, suitable for things like paper.
///
/// Uses perfectly diffuse reflection via [Lambertian reflection][1].
///
/// [1]: crate::brdf::Lambertian
#[derive(Debug, Clone)]
pub struct Matte {
    ambient: Lambertian,
    diffuse: Lambertian,
}

impl Matte {
    /// Construct a new Matte material.
    ///
    /// - `ka` is the ambient reflectance, giving the brightness coefficient of
    ///   ambient light on the object
    /// - `kd` is the diffuse reflectance; the same as `ka`, but for diffuse
    ///   light
    /// - `colour` is the base hue of the material
    pub fn new(ka: f64, kd: f64, colour: Colour) -> Self {
        let ambient = Lambertian::new(ka, colour);
        let diffuse = Lambertian::new(kd, colour);
        Self { ambient, diffuse }
    }
}

impl Material for Matte {
    fn shade(&self, hit: &Intersection) -> Colour {
        let out_dir = -hit.ray.direction;
        let light = self.ambient.rho(hit, out_dir) * hit.world.ambient.radiance(hit);

        hit.world.lights.iter().fold(light, |accum, light| {
            let in_dir = light.direction(hit);
            let angle = hit.normal.dot(in_dir);
            if angle > 0.0 {
                let base_diffuse = self.diffuse.call(hit, out_dir, in_dir);
                accum + base_diffuse * light.radiance(hit) * angle
            } else {
                accum
            }
        })
    }
}
