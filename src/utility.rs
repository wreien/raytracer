//! Various helper utilities used in the raytracer

use image::Rgb;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A three-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new vector with the given coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Calculates the dot product of two vectors.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculates the cross product of two vectors.
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.z * rhs.y - self.y * rhs.x,
        }
    }

    /// The magnitude of the vector.
    pub fn mag(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns the normalised vector.
    pub fn normalise(self) -> Self {
        self / self.mag()
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

/// A two-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Creates a new vector with the given coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the dot product of two vectors.
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The magnitude of the vector.
    pub fn mag(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns the normalised vector.
    pub fn normalise(self) -> Self {
        self / self.mag()
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vec2> for f64 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Self::Output::new(self / rhs.x, self / rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y)
    }
}

/// A point in the RGB colour cube.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn grey() -> Self {
        Self::new(0.5, 0.5, 0.5)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Component-wise floating-point power function
    pub fn powf(self, n: f64) -> Self {
        Self::new(self.r.powf(n), self.g.powf(n), self.b.powf(n))
    }

    /// Component-wise floating-point power function
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.r.powi(n), self.g.powi(n), self.b.powi(n))
    }
}

impl Sub for Colour {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Colour {
    type Output = Colour;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, rhs: Colour) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Div<f64> for Colour {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Div<Colour> for f64 {
    type Output = Colour;
    fn div(self, rhs: Colour) -> Self::Output {
        Self::Output {
            r: self / rhs.r,
            g: self / rhs.g,
            b: self / rhs.b,
        }
    }
}

impl From<Colour> for Rgb<u8> {
    fn from(c: Colour) -> Rgb<u8> {
        let max = c.r.max(c.g.max(c.b));
        let c = if max > 1.0 { c / max } else { c };
        Rgb([
            (c.r * 255.0).min(255.0).max(0.0) as u8,
            (c.g * 255.0).min(255.0).max(0.0) as u8,
            (c.b * 255.0).min(255.0).max(0.0) as u8,
        ])
    }
}

/// An infinite ray, from a given point and with a given direction.
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
