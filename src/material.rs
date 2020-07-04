//! Materials.
//!
//! These are used to shade objects in different manners: by applying different
//! materials you can get an objects behaviour in regards to light to change.
//!
//! See the currently available [BRDFs](crate::brdf) for reflection functions
//! used in materials.

use crate::brdf::{GlossySpecular, Lambertian, BRDF};
use crate::utility::{Colour, Ray};
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
                let shadow = Ray {
                    origin: hit.hit_point,
                    direction: in_dir,
                };
                if !light.in_shadow(shadow, hit.world) {
                    let base_diffuse = self.diffuse.call(hit, in_dir, out_dir);
                    accum + base_diffuse * light.radiance(hit) * angle
                } else {
                    accum
                }
            } else {
                accum
            }
        })
    }
}

/// Phong reflections, suitable for shiny objects like metal.
#[derive(Debug, Clone)]
pub struct Phong {
    ambient: Lambertian,
    diffuse: Lambertian,
    specular: GlossySpecular,
}

impl Phong {
    pub fn new(ka: f64, kd: f64, ks: f64, shininess: f64, colour: Colour) -> Self {
        let ambient = Lambertian::new(ka, colour);
        let diffuse = Lambertian::new(kd, colour);
        let specular = GlossySpecular::new(ks, shininess, colour);
        Self {
            ambient,
            diffuse,
            specular,
        }
    }
}

impl Material for Phong {
    fn shade(&self, hit: &Intersection) -> Colour {
        let out_dir = -hit.ray.direction;
        let light = self.ambient.rho(hit, out_dir) * hit.world.ambient.radiance(hit);

        hit.world.lights.iter().fold(light, |accum, light| {
            let in_dir = light.direction(hit);
            let angle = hit.normal.dot(in_dir);
            if angle > 0.0 {
                let shadow = Ray {
                    origin: hit.hit_point,
                    direction: in_dir,
                };
                if !light.in_shadow(shadow, hit.world) {
                    let base_diffuse = self.diffuse.call(hit, in_dir, out_dir);
                    let base_specular = self.specular.call(hit, in_dir, out_dir);
                    accum + (base_diffuse + base_specular) * light.radiance(hit) * angle
                } else {
                    accum
                }
            } else {
                accum
            }
        })
    }
}
