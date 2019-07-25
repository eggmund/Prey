use ggez::nalgebra as na;
use na::{Point2, Vector2};

use rand::{rngs::ThreadRng, Rng};

// For boids
pub fn rand_vector2(
    rand_thread: &mut ThreadRng,
    min_mag: f64,
    max_mag: f64,
    min_angle: f64,
    max_angle: f64,
) -> Vector2<f64> {
    let mag = rand_thread.gen_range(min_mag, max_mag);
    let angle = rand_thread.gen_range(min_angle, max_angle);

    get_components(mag, angle)
}

#[inline]
pub fn get_components(mag: f64, angle: f64) -> Vector2<f64> {
    Vector2::new(mag * angle.cos(), mag * angle.sin())
}

#[inline]
pub fn get_angle(vec: &Vector2<f64>) -> f64 {
    vec.y.atan2(vec.x)
}

#[inline]
pub fn get_magnitude(vec: &Vector2<f64>) -> f64 {
    get_magnitude_squared(vec).sqrt()
}

#[inline]
pub fn get_magnitude_squared(vec: &Vector2<f64>) -> f64 {
    vec.x.powi(2) + vec.y.powi(2)
}

#[inline]
pub fn point_is_in_radius(
    center_pos: &Point2<f64>,
    point: &Point2<f64>,
    radius_squared: f64,
) -> bool {
    let dist_vec = *point - *center_pos;
    get_magnitude_squared(&dist_vec) <= radius_squared
}

#[inline]
pub fn limit_vector_mag(vec: Vector2<f64>, max_squared: f64) -> Vector2<f64> {
    let mag_sqr = get_magnitude_squared(&vec);

    if mag_sqr >= max_squared {
        // Then limit
        set_vector_mag(vec, max_squared)
    } else {
        vec
    }
}

#[inline]
pub fn set_vector_mag(vec: Vector2<f64>, mag: f64) -> Vector2<f64> {
    let angle = get_angle(&vec);
    get_components(mag, angle)
}

#[inline]
pub fn clamp_vector_mag(vec: Vector2<f64>, min: f64, max: f64) -> Vector2<f64> {
    let mag = get_magnitude_squared(&vec).sqrt();

    if mag >= max {
        set_vector_mag(vec, max)
    } else if mag <= min {
        set_vector_mag(vec, min)
    } else {
        vec
    }
}
