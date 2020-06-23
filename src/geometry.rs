//! Different kinds of objects in the world.

use crate::utility::{Ray, Shader, Vec3};

/// Used to ignore rounding errors, and prevent contact with camera.
const EPSILON: f64 = 0.0001;

/// Interface trait for objects with geometry.
///
/// If the given ray hits the geometry, writes into `shader` and returns
/// the distance along the ray the collision occurred. Otherwise returns `None`.
pub trait Geometry {
    fn hit(&self, ray: &Ray, shader: &mut Shader) -> Option<f64>;
}

#[derive(Debug)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl Geometry for Plane {
    fn hit(&self, ray: &Ray, shader: &mut Shader) -> Option<f64> {
        let t =
            (self.point - ray.origin).dot(self.normal) / ray.direction.dot(self.normal);
        if t > EPSILON {
            shader.normal = self.normal;
            shader.hit_point = ray.origin + t * ray.direction;
            Some(t)
        } else {
            None
        }
    }
}

impl Geometry for Sphere {
    fn hit(&self, ray: &Ray, shader: &mut Shader) -> Option<f64> {
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
                shader.normal = (offset + t * ray.direction) / self.radius;
                shader.hit_point = ray.origin + t * ray.direction;
                return Some(t);
            }

            let t = (-b + e) / denominator;
            if t > EPSILON {
                shader.normal = (offset + t * ray.direction) / self.radius;
                shader.hit_point = ray.origin + t * ray.direction;
                return Some(t);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plane_hit() {
        let mut shader = Shader::new();
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 100.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let plane = Plane {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, -1.0),
        };

        let result = plane.hit(&ray, &mut shader);
        assert_eq!(result, Some(100.0));
        assert_eq!(shader.hit_point, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn sphere_hit() {
        let mut shader = Shader::new();
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 100.0),
            direction: Vec3::new(0.0, 0.0, -1.0),
        };
        let sphere = Sphere {
            centre: Vec3::new(0.0, 0.0, 0.0),
            radius: 50.0,
        };

        let result = sphere.hit(&ray, &mut shader);
        assert_eq!(result, Some(50.0));
        assert_eq!(shader.hit_point, Vec3::new(0.0, 0.0, 50.0));
    }
}
