//! Bidirectional reflectance distribution functions.
//!
//! These are used by [materials](crate::material) to determine how to render
//! luminence and shading. Although there's a main [`BRDF`] trait, most
//! materials will use a particular BRDF rather than generically templating on
//! it.

use crate::utility::{Colour, Vec3};
use crate::world::Intersection;

use std::f64::consts;

/// A BRDF function.
///
/// All bidirectional reflectance distribution functions will implement this
/// trait, but generally materials should just use the particular BRDF they need
/// directly.
pub trait BRDF {
    /// Call the BRDF. This returns the contribution of the reflected irradiance
    /// from `in_dir` in the direction `out_dir`.
    fn call(&self, hit: &Intersection, in_dir: Vec3, out_dir: Vec3) -> Colour;

    /// The bihemispherial reflectance ρ for `out_dir`
    fn rho(&self, hit: &Intersection, out_dir: Vec3) -> Colour;
}

/// Perfect diffuse reflection.
///
/// This is a good approximation for dull, matte materials like paper.
#[derive(Debug, Clone)]
pub struct Lambertian {
    rho: Colour,
}

impl Lambertian {
    pub fn new(reflectance: f64, colour: Colour) -> Self {
        let rho = reflectance * colour;
        Self { rho }
    }
}

impl BRDF for Lambertian {
    fn call(&self, _hit: &Intersection, _in_dir: Vec3, _out_dir: Vec3) -> Colour {
        self.rho * consts::FRAC_1_PI
    }
    fn rho(&self, _hit: &Intersection, _out_dir: Vec3) -> Colour {
        self.rho
    }
}
