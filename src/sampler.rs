//! Various techniques for sampling.
//!
//! Used for e.g. antialiasing, lighting, shading, ambient occlusion, etc.
//!
//! # TODO
//!
//! This whole structure is dubious; I really don't want to do it the same way
//! as the book does. As such, this is but a temporary holdout until I can
//! implement some of the more complex examples using sampling to see what
//! structure will suit best.
//!
//! In the meantime, this will suffice.

use crate::utility::{Vec2, Vec3};
use rand::prelude::*;
use std::{f64, fmt::Debug};

/// Number of sets of samples to generate.
///
/// **TODO** Maybe move this to a variable someday.
const NUM_SETS: usize = 83;

/// An abstract sample generator.
///
/// Items implementing this trait can be used to generate samples for any of the
/// following purposes:
///
/// - samples in a unit square
/// - samples on a unit disc
/// - samples on a three-dimensional unit hemisphere
///
/// Each of the primary functions provided by this trait return [`Samples`]:
/// this is a custom container representing a number of sets of samples.
///
/// # Example
///
/// ```
/// fn get_generator() -> impl Generator;
/// # fn get_generator() -> impl Generator { Jittered::new(25) }
///
/// let gen = get_generator();
/// let mut sample_set = gen.gen_square_samples();
/// let s = sample_set.get_next();
/// ```
pub trait Generator: Debug {
    /// The number of samples in each set.
    fn num_samples(&self) -> usize;

    /// The number of sets generated.
    fn num_sets(&self) -> usize {
        NUM_SETS
    }

    /// Generate a single set of samples on the unit square.
    ///
    /// This should generally not be used; prefer instead `gen_square_samples`.
    fn new_square_set(&self) -> Vec<Vec2>;

    /// Generates samples on the unit square.
    ///
    /// Each sample is between points `(0, 0)` and `(1, 1)`.
    fn gen_square_samples(&self) -> Samples<Vec2> {
        let samples = (0..self.num_sets())
            .map(|_| self.new_square_set())
            .collect();
        Samples::new(self.num_samples(), samples)
    }

    /// Generates samples on the unit disc.
    ///
    /// Each sample is distributed on the disc with centre `(0, 0)` and radius
    /// `1`.
    fn gen_disc_samples(&self) -> Samples<Vec2> {
        let samples = (0..self.num_sets())
            .map(|_| self.new_square_set())
            .map(map_square_to_unit_disk)
            .collect();
        Samples::new(self.num_samples(), samples)
    }

    /// Generates samples on the unit hemisphere.
    ///
    /// Each sample is placed on the hemisphere with centre `(0, 0, 0)`, radius
    /// `1`, and `z ≥ 0`.
    ///
    /// The exponent `e` is used to configure the cosine distribution: higher
    /// values of `e` cause the samples to be distributed closer to the top
    /// of the hemisphere. The samples are evenly distributed when `e =
    /// 0.0`. The value of `e` must be non-negative.
    fn gen_hemisphere_samples(&self, e: f64) -> Samples<Vec3> {
        let samples = (0..self.num_sets())
            .map(|_| self.new_square_set())
            .map(|s| map_square_to_hemisphere(s, e))
            .collect();
        Samples::new(self.num_samples(), samples)
    }
}

/// A set of jittered samples.
///
/// The unit square is first divided up into a grid of `num_samples` tiles.
/// Each sample is then randomly placed somewhere on that grid.
#[derive(Debug, Clone)]
pub struct Jittered {
    num_samples: usize,
    n: usize,
}

impl Jittered {
    /// Creates a new generator.
    ///
    /// The parameter `num_samples` must be a square number.
    pub fn new(num_samples: usize) -> Self {
        let n = (num_samples as f64).sqrt() as usize;
        assert!(
            n * n == num_samples,
            "Jittered requires a square number of samples"
        );

        Self { num_samples, n }
    }
}

impl Generator for Jittered {
    fn num_samples(&self) -> usize {
        self.num_samples
    }

    fn new_square_set(&self) -> Vec<Vec2> {
        let mut rng = thread_rng();
        let mut s = vec![];
        for x in 0..self.n {
            for y in 0..self.n {
                let x = x as f64 + rng.gen::<f64>();
                let y = y as f64 + rng.gen::<f64>();
                s.push(Vec2::new(x, y) / (self.n as f64));
            }
        }
        s
    }
}

/// A container of sample sets.
///
/// You can continuously create new sample sets by calling `get_next`.
/// These samples will never run out: you may keep calling the function forever.
/// However, there are only a fixed number of sets that may be generated.
///
/// The distribution of the samples depends on the [`Generator`] used to
/// construct this set. Using an appropriate [`Generator`] is the only method of
/// constructing a `SampleSet`.
///
/// # Example
///
/// ```no_run
/// let mut sample_set = Jittered::new(25).gen_square_samples();
/// loop {
///     // Generate a set 25 random samples in a unit square
///     let samples = sample_set.get_next();
///     // ...
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Samples<T> {
    samples: Vec<Vec<T>>,
    num_samples: usize,
    count: usize,
}

impl<T: Clone> Samples<T> {
    fn new(num_samples: usize, samples: Vec<Vec<T>>) -> Self {
        assert!(num_samples == samples[0].len());
        Self {
            num_samples,
            samples,
            count: 0,
        }
    }

    /// Returns the number of samples in a given sample set.
    ///
    /// Equivalent to `self.get_next().len()`, but doesn't actually require
    /// getting an sample set.
    pub fn num_samples(&self) -> usize {
        self.num_samples
    }

    /// Get the next random sample set.
    ///
    /// This never runs out.
    pub fn get_next(&mut self) -> &Vec<T> {
        // if we only have one set, just generate it over and over
        // e.g. for Regular
        if self.samples.len() > 1 {
            self.count = (self.count + 1) % self.samples.len();
            if self.count == 0 {
                self.samples.shuffle(&mut thread_rng());
            }
        }
        self.samples.get(self.count).unwrap()
    }
}

/// Given a sample on the unit square, transform it to lie on the unit disk.
fn square_to_unit_disk(sample: Vec2) -> Vec2 {
    let Vec2 { x, y } = 2.0 * sample - Vec2::new(1.0, 1.0);

    let r;
    let phi;
    if x > -y {
        if x > y {
            r = x;
            phi = x / y;
        } else {
            r = y;
            phi = 2.0 - x / y;
        }
    } else {
        if x < y {
            r = -x;
            phi = 4.0 + y / x;
        } else if y != 0.0 {
            r = -y;
            phi = 6.0 - x / y;
        } else {
            r = -y;
            phi = 0.0;
        }
    };

    let phi = phi * f64::consts::FRAC_PI_4;
    Vec2::new(r * phi.cos(), r * phi.sin())
}

fn map_square_to_unit_disk(samples: Vec<Vec2>) -> Vec<Vec2> {
    samples.into_iter().map(square_to_unit_disk).collect()
}

/// Given a sample on the unit square, transform it to lie on the unit
/// hemisphere with z ≥ 0, according to the cosine distribution with exponent
/// `e`.
fn square_to_hemisphere(sample: Vec2, e: f64) -> Vec3 {
    let Vec2 { x, y } = sample;
    let phi = 2.0 * f64::consts::PI * x;
    let cos_theta = (1.0 - y).powf(1.0 / (e + 1.0));
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    Vec3 {
        x: sin_theta * phi.cos(),
        y: sin_theta * phi.sin(),
        z: cos_theta,
    }
}

fn map_square_to_hemisphere(samples: Vec<Vec2>, e: f64) -> Vec<Vec3> {
    samples
        .into_iter()
        .map(|s| square_to_hemisphere(s, e))
        .collect()
}
