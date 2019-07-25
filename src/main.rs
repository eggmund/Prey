mod boid;
mod tools;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use na::{Point2, Vector2};

use rand::{rngs::ThreadRng, Rng};

use std::cell::RefCell;
use std::f64::consts::PI;

use crate::boid::{
    Boid, BoidType, BOID_MAX_SPEED, BOID_MIN_SPEED, BOID_SENSORY_RADIUS_SQR, BOID_SIZE, BOUNDING_RADIUS_SQR
};

const TWO_PI: f64 = PI * 2.0;
const ALIGNMENT_MULT: f64 = 5.0;
const COHESION_MULT: f64 = 10.0;
const SEPARATION_MULT: f64 = 200.0;

const PREY_ESCAPE_MULT: f64 = -550.0;
const PREDATOR_CHASE_MULT: f64 = 1750.0;

const SCREEN_DIMS: [f32; 2] = [800.0, 600.0];

const N: usize = 512;

struct MainState {
    boids: Vec<RefCell<Boid>>,
    spawn_rand_thread: ThreadRng,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            boids: Vec::with_capacity(N),
            spawn_rand_thread: rand::thread_rng(),
        };

        for _ in 0..N {
            let rand_vel = tools::rand_vector2(
                &mut s.spawn_rand_thread,
                BOID_MIN_SPEED,
                BOID_MAX_SPEED,
                0.0,
                TWO_PI,
            );

            let rand_pos = Point2::new(
                s.spawn_rand_thread.gen_range(0.0, 800.0),
                s.spawn_rand_thread.gen_range(0.0, 600.0),
            );

            s.boids
                .push(RefCell::new(Boid::new(BoidType::Prey, rand_pos, rand_vel)));
        }

        s.boids.push(RefCell::new(Boid::new(
            BoidType::Predator,
            Point2::new(400.0, 300.0),
            Vector2::new(0.0, 0.0),
        )));

        Ok(s)
    }

    fn teleport_edges(&mut self) {
        for boid in self.boids.iter_mut() {
            let mut boid = boid.borrow_mut();
            if boid.position.x <= -BOID_SIZE[0] as f64 {
                boid.position.x = (SCREEN_DIMS[0] + BOID_SIZE[0] / 2.0) as f64;
            }
            if boid.position.x > (SCREEN_DIMS[0] + BOID_SIZE[0] / 2.0) as f64 {
                boid.position.x = 0.0;
            }

            if boid.position.y <= -BOID_SIZE[0] as f64 {
                boid.position.y = (SCREEN_DIMS[1] + BOID_SIZE[0] / 2.0) as f64;
            }
            if boid.position.y > (SCREEN_DIMS[1] + BOID_SIZE[0] / 2.0) as f64 {
                boid.position.y = 0.0;
            }
        }
    }

    fn bounce_edges(&mut self) {
        for boid in self.boids.iter_mut() {
            let mut boid = boid.borrow_mut();
            if boid.position.x <= BOID_SIZE[0] as f64
                || boid.position.x > (SCREEN_DIMS[0] - BOID_SIZE[0] / 2.0) as f64
            {
                boid.velocity.x = -boid.velocity.x;

                if boid.position.x <= BOID_SIZE[0] as f64 {
                    boid.position.x += 1.0; // Move away from edge
                } else {
                    boid.position.x -= 1.0;
                }
            }

            if boid.position.y <= BOID_SIZE[0] as f64
                || boid.position.y > (SCREEN_DIMS[1] - BOID_SIZE[0] / 2.0) as f64
            {
                boid.velocity.y = -boid.velocity.y;

                if boid.position.y <= BOID_SIZE[1] as f64 {
                    boid.position.y += 1.0; // Move away from edge
                } else {
                    boid.position.y -= 1.0;
                }
            }
        }
    }

    #[inline]
    fn check_boids_colliding(pos1: &Point2<f64>, pos2: &Point2<f64>) -> bool {
        let dist = *pos2 - *pos1;
        tools::get_magnitude_squared(&dist) < BOUNDING_RADIUS_SQR
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx));

        self.bounce_edges();

        for i in 0..self.boids.len() {
            let mut current = self.boids[i].borrow_mut();
            current.acceleration = Vector2::new(0.0, 0.0);

            let mut total_vel = Vector2::new(0.0, 0.0);
            let mut total_pos = Vector2::new(0.0, 0.0);

            let mut total_in_sensory_radius = 0_usize;

            for j in 0..self.boids.len() {
                if i != j {
                    let other = self.boids[j].borrow();

                    if other.b_type == BoidType::Predator && Self::check_boids_colliding(&current.position, &other.position) {
                        current.b_type = BoidType::Predator;
                    }

                    let mut diff_vector = other.position - current.position;
                    let distance_to_other_sqrd = tools::get_magnitude_squared(&diff_vector);
                    if distance_to_other_sqrd < BOID_SENSORY_RADIUS_SQR {
                        // if in sensory radius
                        // Alignment
                        total_vel += other.velocity;
                        // Cohesion
                        total_pos += Vector2::new(other.position.x, other.position.y);

                        // Seperation
                        diff_vector /= distance_to_other_sqrd.sqrt(); // Normalise

                        if current.b_type == other.b_type {
                            current.acceleration -= diff_vector * SEPARATION_MULT;
                        } else if current.b_type == BoidType::Predator
                            && other.b_type == BoidType::Prey
                        {
                            current.acceleration += diff_vector * PREDATOR_CHASE_MULT; // Add to acceleration
                        } else {
                            current.acceleration += diff_vector * PREY_ESCAPE_MULT; // Add to acceleration
                        }

                        total_in_sensory_radius += 1;
                    }
                }
            }

            if total_in_sensory_radius > 0 {
                let desired_vel = total_vel / total_in_sensory_radius as f64;
                let desired_pos = total_pos / total_in_sensory_radius as f64;

                // Alignment
                current.acceleration += (desired_vel - current.velocity) * ALIGNMENT_MULT;
                // Cohesion
                current.acceleration += (desired_pos
                    - Vector2::new(current.position.x, current.position.y))
                    * COHESION_MULT;
            }

            current.update(dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for boid in self.boids.iter() {
            boid.borrow().draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Prey", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
