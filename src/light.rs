//! Emitters and ambient lights.

use crate::utility::{Colour, Ray, Vec3};
use crate::world::{Intersection, World};

use std::fmt::Debug;

pub trait Light: Debug {
    fn direction(&self, hit: &Intersection) -> Vec3;
    fn radiance(&self, hit: &Intersection) -> Colour;
    fn in_shadow(&self, ray: Ray, world: &World) -> bool;
}

/// Ambient lighting to give a base diffuse shading.
///
/// We don't want everything black, do we? This is as simple a solution as can
/// be.
#[derive(Debug)]
pub struct Ambient {
    pub scale: f64,
    pub colour: Colour,
}

impl Ambient {
    pub fn new(scale: f64) -> Self {
        Self::with_colour(scale, Colour::white())
    }

    pub fn with_colour(scale: f64, colour: Colour) -> Self {
        Self { scale, colour }
    }
}

impl Light for Ambient {
    fn direction(&self, _hit: &Intersection) -> Vec3 {
        // unused, in theory
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn radiance(&self, _hit: &Intersection) -> Colour {
        self.scale * self.colour
    }

    fn in_shadow(&self, _ray: Ray, _world: &World) -> bool {
        false
    }
}

/// A light emitting from an infinitely small point.
///
/// This implementation has no distance attenuation.
#[derive(Debug)]
pub struct PointLight {
    pub scale: f64,
    pub colour: Colour,
    pub location: Vec3,
}

impl PointLight {
    pub fn new(scale: f64, location: Vec3) -> Self {
        Self::with_colour(scale, location, Colour::white())
    }

    pub fn with_colour(scale: f64, location: Vec3, colour: Colour) -> Self {
        Self {
            scale,
            location,
            colour,
        }
    }
}

impl Light for PointLight {
    fn direction(&self, hit: &Intersection) -> Vec3 {
        (self.location - hit.hit_point).normalise()
    }

    fn radiance(&self, _hit: &Intersection) -> Colour {
        // no distance attenuation, so basically just ambient
        self.scale * self.colour
    }

    fn in_shadow(&self, ray: Ray, world: &World) -> bool {
        let offset = self.location - ray.origin;
        let distance_squared = offset.dot(offset);

        world.objects.iter().any(|obj| match obj.hit(&ray) {
            Some((t, _)) => t * t < distance_squared,
            _ => false,
        })
    }
}
