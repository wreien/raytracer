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

/// An axis-aligned cuboid.
///
/// # TODO
///
/// Add support for being, well, not axis-aligned.
#[derive(Debug)]
pub struct Cuboid {
    /// Min point.
    pub min: Vec3,
    /// Max point.
    pub max: Vec3,
    pub colour: Colour,
}

impl Cuboid {
    pub fn new(min: Vec3, max: Vec3, colour: Colour) -> Self {
        Self { min, max, colour }
    }

    pub fn with_size(origin: Vec3, size: Vec3, colour: Colour) -> Self {
        Self {
            min: origin,
            max: origin + size,
            colour,
        }
    }
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

impl Geometry for Cuboid {
    /// Calculates the intersection point using slab intersection.
    fn hit(&self, ray: &Ray) -> Option<Intersection> {
        // TODO: include this in the ray itself?
        let invdir = 1.0 / ray.direction;

        let t_x1 = (self.min.x - ray.origin.x) * invdir.x;
        let t_x2 = (self.max.x - ray.origin.x) * invdir.x;
        let t_y1 = (self.min.y - ray.origin.y) * invdir.y;
        let t_y2 = (self.max.y - ray.origin.y) * invdir.y;
        let t_z1 = (self.min.z - ray.origin.z) * invdir.z;
        let t_z2 = (self.max.z - ray.origin.z) * invdir.z;

        let t_xn = t_x1.min(t_x2);
        let t_xf = t_x1.max(t_x2);
        let t_yn = t_y1.min(t_y2);
        let t_yf = t_y1.max(t_y2);
        let t_zn = t_z1.min(t_z2);
        let t_zf = t_z1.max(t_z2);

        let t_min = t_xn.max(t_yn.max(t_zn));
        let t_max = t_xf.min(t_yf.min(t_zf));

        if t_min < t_max {
            let t = if t_min < 0.0 { t_max } else { t_min };
            Some(Intersection {
                ray: ray.clone(),
                t,
                colour: self.colour,
            })
        } else {
            None
        }
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        let centre = (self.min + self.max) * 0.5;
        let offset = pos - centre;
        let divisor = (self.min - self.max) * 0.5;
        let bias = 1.0 + EPSILON;

        Vec3 {
            x: (offset.x / divisor.x.abs() * bias).trunc(),
            y: (offset.y / divisor.y.abs() * bias).trunc(),
            z: (offset.z / divisor.z.abs() * bias).trunc(),
        }
        .normalise()
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
