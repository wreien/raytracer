//! Various kinds of raytracers

use crate::utility::{Colour, Ray};
use crate::world::World;

/// An abstract raytracer.
///
/// Allows using different techniques and methods to get the colour of a ray.
pub trait Tracer {
    /// Returns the colour of the ray's impact location.
    fn trace_ray(&self, world: &World, ray: Ray) -> Colour;
}

/// A very simple tracer for a single object.
///
/// If the ray hits the first object in the world, colours the pixel red,
/// else it colours the pixel black. Can't get any simpler ☺
#[derive(Debug)]
pub struct SimpleTracer {}

impl Tracer for SimpleTracer {
    fn trace_ray(&self, world: &World, ray: Ray) -> Colour {
        if let Some(_) = world.objects[0].hit(&ray) {
            Colour::red()
        } else {
            Colour::black()
        }
    }
}

/// A slightly more interesting tracer for multiple objects.
///
/// Uses `world::World::hit_objects` to find the closest object in the world.
#[derive(Debug)]
pub struct MultipleObjectTracer {}

impl Tracer for MultipleObjectTracer {
    fn trace_ray(&self, world: &World, ray: Ray) -> Colour {
        if let Some(intersection) = world.hit_objects(ray) {
            intersection.colour
        } else {
            world.background
        }
    }
}
