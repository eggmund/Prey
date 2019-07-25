use ggez::graphics::{self, DrawMode, DrawParam, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use na::{Point2, Vector2};

use crate::tools;

pub const BOID_SIZE: [f32; 2] = [4.0, 4.0]; //[5.0, 2.0];
pub const BOUNDING_RADIUS_SQR: f32 = (BOID_SIZE[0] * BOID_SIZE[0]) + (BOID_SIZE[1] * BOID_SIZE[1]); // Bouding circle, requires that rectangle is not too rectanglish

pub const BOID_SENSORY_RADIUS: f32 = 80.0;
pub const BOID_SENSORY_RADIUS_SQR: f32 = BOID_SENSORY_RADIUS * BOID_SENSORY_RADIUS;
pub const BOID_MAX_SPEED: f32 = 100.0;
pub const BOID_MIN_SPEED: f32 = 30.0;
pub const PREDATOR_MAX_SPEED: f32 = 130.0;

pub struct Boid {
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub b_type: BoidType,
}

impl Default for Boid {
    fn default() -> Boid {
        Boid {
            position: Point2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            b_type: BoidType::Prey,
        }
    }
}

impl Boid {
    pub fn new(b_type: BoidType, position: Point2<f32>, velocity: Vector2<f32>) -> Boid {
        Boid {
            b_type,
            position,
            velocity,
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.velocity += self.acceleration * dt;
        self.velocity = tools::clamp_vector_mag(
            self.velocity,
            BOID_MIN_SPEED,
            match self.b_type {
                BoidType::Predator => PREDATOR_MAX_SPEED,
                _ => BOID_MAX_SPEED,
            },
        );
    }

    pub fn draw(&self, ctx: &mut Context, draw_sense: bool) -> GameResult {
        let boid_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(0.0, 0.0, BOID_SIZE[0], BOID_SIZE[1]),
            match self.b_type {
                BoidType::Prey => [0.1, 1.0, 0.1, 1.0].into(),
                BoidType::Predator => [1.0, 0.1, 0.1, 1.0].into(),
            },
        )?;

        if draw_sense {
            self.draw_sensory_radius(ctx)?;
        }

        graphics::draw(
            ctx,
            &boid_mesh,
            DrawParam::new()
                .dest(self.position)
                .rotation(tools::get_angle(&self.velocity)),
        )
    }

    fn draw_sensory_radius(&self, ctx: &mut Context) -> GameResult {
        let circ = Mesh::new_circle(
            ctx,
            DrawMode::stroke(2.0),
            self.position,
            BOID_SENSORY_RADIUS,
            0.1,
            [1.0, 1.0, 1.0, 0.3].into(),
        )?;

        graphics::draw(ctx, &circ, DrawParam::new())
    }
}

#[derive(Eq, PartialEq)]
pub enum BoidType {
    Prey,
    Predator,
}
