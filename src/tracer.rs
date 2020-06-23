//! Various kinds of raytracers

use crate::utility::{Colour, Shader, Ray};
use crate::geometry::Geometry;
use crate::world::World;

pub trait Tracer {
    fn trace_ray(&self, world: &World, ray: Ray) -> Colour;
}

#[derive(Debug)]
pub struct SimpleTracer {}

impl Tracer for SimpleTracer {
    fn trace_ray(&self, world: &World, ray: Ray) -> Colour {
        let mut shader = Shader::new();
        if let Some(_) = world.sphere.hit(&ray, &mut shader) {
            Colour::red()
        } else {
            Colour::black()
        }
    }
}
