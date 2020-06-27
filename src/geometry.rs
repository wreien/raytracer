//! Different kinds of objects in the world.

use crate::utility::{Colour, Ray, Vec3};
use std::fmt;

/// Used to ignore rounding errors, and prevent contact with camera.
const EPSILON: f64 = 0.0001;

/// Interface trait for objects with geometry.
///
/// If the given ray hits the geometry, writes into `shader` and returns
/// the distance along the ray the collision occurred. Otherwise returns `None`.
pub trait Geometry: fmt::Debug {
    /// If the ray will collide with this geometry, returns details on the
    /// intersection.
    fn hit(&self, ray: &Ray) -> Option<Intersection>;

    /// Returns the normal on the geometry at the given point.
    ///
    /// Assumes the point is in fact (approximately) on the geometry, though it
    /// is largely unlikely to matter.
    fn normal(&self, pos: Vec3) -> Vec3;
}

/// The result of an intersection.
pub struct Intersection {
    pub ray: Ray,
    pub t: f64,
    pub colour: Colour, // temporary
}

/// An infinite plane.
#[derive(Debug)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub colour: Colour,
}

/// A simple sphere.
#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub colour: Colour,
}

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let offset = self.point - ray.origin;
        let t = offset.dot(self.normal) / ray.direction.dot(self.normal);
        if t > EPSILON {
            Some(Intersection {
                ray: ray.clone(),
                t,
                colour: self.colour,
            })
        } else {
            None
        }
    }

    fn normal(&self, _pos: Vec3) -> Vec3 {
        self.normal
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        let offset = ray.origin - self.centre;

        // quadratic equation for "ax^2 + bx + c = 0"
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * offset.dot(ray.direction);
        let c = offset.dot(offset) - self.radius * self.radius;
        let discriminator = b * b - 4.0 * a * c;

        if discriminator >= 0.0 {
            let e = discriminator.sqrt();
            let denominator = 2.0 * a;

            let t = (-b - e) / denominator;
            if t > EPSILON {
                return Some(Intersection {
                    ray: ray.clone(),
                    t,
                    colour: self.colour,
                });
            }

            let t = (-b + e) / denominator;
            if t > EPSILON {
                return Some(Intersection {
                    ray: ray.clone(),
                    t,
                    colour: self.colour,
                });
            }
        }

        None
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        (pos - self.centre).normalise()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plane_hit() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 100.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let plane = Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
            colour: Colour::black(),
        };

        let result = plane.hit(&ray);
        assert!(result.is_some());
        assert_eq!(result.unwrap().t, 100.0);
    }

    #[test]
    fn sphere_hit() {
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 100.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let sphere = Sphere {
            centre: Vec3::new(0.0, 0.0, 0.0),
            radius: 50.0,
            colour: Colour::black(),
        };

        let result = sphere.hit(&ray);
        assert!(result.is_some());
        assert_eq!(result.unwrap().t, 50.0);
    }
}
