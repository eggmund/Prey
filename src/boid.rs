use ggez::graphics::{self, DrawMode, DrawParam, Mesh, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use na::{Point2, Vector2};

use crate::tools;

pub const BOID_SIZE: [f32; 2] = [4.0, 4.0]; //[5.0, 2.0];
pub const BOUNDING_RADIUS_SQR: f64 =
    ((BOID_SIZE[0] * BOID_SIZE[0]) + (BOID_SIZE[1] * BOID_SIZE[1])) as f64; // Bouding circle, requires that rectangle is not too rectanglish

pub const BOID_SENSORY_RADIUS: f64 = 70.0;
pub const BOID_SENSORY_RADIUS_SQR: f64 = BOID_SENSORY_RADIUS * BOID_SENSORY_RADIUS;
pub const BOID_MAX_SPEED: f64 = 100.0;
pub const BOID_MIN_SPEED: f64 = 50.0;

pub struct Boid {
    pub position: Point2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
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
    pub fn new(b_type: BoidType, position: Point2<f64>, velocity: Vector2<f64>) -> Boid {
        Boid {
            b_type,
            position,
            velocity,
            ..Default::default()
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
        self.velocity += self.acceleration * dt;
        self.velocity = tools::clamp_vector_mag(self.velocity, BOID_MIN_SPEED, BOID_MAX_SPEED);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        // let boid_mesh = Mesh::new_ellipse(
        //     ctx,
        //     DrawMode::stroke(2.0),
        //     Point2::new(0.0, 0.0),
        //     BOID_SIZE[0],
        //     BOID_SIZE[1],
        //     0.1,
        //     match self.b_type {
        //         BoidType::Prey => [0.1, 1.0, 0.1, 1.0].into(),
        //         BoidType::Predator => [1.0, 0.1, 0.1, 1.0].into()
        //     }
        // )?;

        let boid_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(0.0, 0.0, BOID_SIZE[0], BOID_SIZE[1]),
            match self.b_type {
                BoidType::Prey => [0.1, 1.0, 0.1, 1.0].into(),
                BoidType::Predator => [1.0, 0.1, 0.1, 1.0].into(),
            },
        )?;

        graphics::draw(
            ctx,
            &boid_mesh,
            DrawParam::new()
                .dest(Point2::new(self.position.x as f32, self.position.y as f32))
                .rotation(tools::get_angle(&self.velocity) as f32),
        )
    }
}

#[derive(Eq, PartialEq)]
pub enum BoidType {
    Prey,
    Predator,
}
