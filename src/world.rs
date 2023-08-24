use crate::vec2::Vec2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}
#[wasm_bindgen]
impl Boid {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel }
    }
}

#[wasm_bindgen]
pub struct World {
    pub dims: Vec2,
    boids: Vec<Boid>,
    pub protect_range: f64,
    avoid_factor: f64,
    pub visible_range: f64,
    align_factor: f64,
    cohesion_factor: f64,
    pub margin: f64,
    turn_factor: f64,
    max_speed: f64,
    min_speed: f64,
}
#[wasm_bindgen]
impl World {
    pub fn new(
        dims: Vec2,
        protect_range: f64,
        avoid_factor: f64,
        visible_range: f64,
        align_factor: f64,
        cohesion_factor: f64,
        margin: f64,
        turn_factor: f64,
        max_speed: f64,
        min_speed: f64,
    ) -> Self {
        World {
            dims,
            boids: Vec::new(),
            protect_range,
            avoid_factor,
            visible_range,
            align_factor,
            cohesion_factor,
            margin,
            turn_factor,
            max_speed,
            min_speed,
        }
    }
    pub fn add_boid(&mut self, pos: Vec2, vel: Vec2) {
        self.boids.push(Boid::new(pos, vel));
    }
    pub fn num_boids(&self) -> usize {
        self.boids.len()
    }
    pub fn get_boid(&self, i: usize) -> Boid {
        self.boids[i]
    }

    // debug functions
    pub fn max_boid_speed(&self) -> f64 {
        let mut mx = 0.0;
        for boid in self.boids.iter() {
            mx = f64::max(mx, boid.vel.length());
        }
        mx
    }
    pub fn min_boid_speed(&self) -> f64 {
        let mut mn = f64::INFINITY;
        for boid in self.boids.iter() {
            mn = f64::min(mn, boid.vel.length());
        }
        mn
    }

    fn separation(&self, i: usize) -> Vec2 {
        let mut close = Vec2::zero();
        for j in 0..self.boids.len() {
            if i == j {
                continue;
            }
            let dis = self.boids[j].pos.distance(&self.boids[i].pos);
            if dis < self.protect_range {
                close -= self.boids[j].pos - self.boids[i].pos;
            }
        }
        close * self.avoid_factor
    }
    fn alignment(&self, i: usize) -> Vec2 {
        let mut vel_avg = Vec2::zero();
        let mut neighbours = 0;
        for j in 0..self.boids.len() {
            if i == j {
                continue;
            }
            let dis = self.boids[i].pos.distance(&self.boids[j].pos);
            if dis < self.visible_range {
                neighbours += 1;
                vel_avg += self.boids[j].vel;
            }
        }
        if neighbours == 0 {
            return Vec2::zero();
        }
        vel_avg /= neighbours as f64;
        (vel_avg - self.boids[i].vel) * self.align_factor
    }
    fn cohesion(&self, i: usize) -> Vec2 {
        let mut pos_avg = Vec2::zero();
        let mut neighbours = 0;
        for j in 0..self.boids.len() {
            if i == j {
                continue;
            }
            let dis = self.boids[i].pos.distance(&self.boids[j].pos);
            if dis < self.visible_range {
                neighbours += 1;
                pos_avg += self.boids[j].pos;
            }
        }
        if neighbours == 0 {
            return Vec2::zero();
        }
        pos_avg /= neighbours as f64;
        (pos_avg - self.boids[i].pos) * self.cohesion_factor
    }
    fn handle_margins(&self, i: usize) -> Vec2 {
        let pos = self.boids[i].pos;
        let mut ret = Vec2::zero();

        if pos.x < self.margin {
            // outside left border
            ret.x = self.turn_factor;
        } else if pos.x > self.dims.x - self.margin {
            // outside right border
            ret.x = -self.turn_factor
        }
        if pos.y < self.margin {
            // outside top margin
            ret.y = self.turn_factor;
        } else if pos.y > self.dims.y - self.margin {
            // outside bottom margin
            ret.y = -self.turn_factor
        }
        ret
    }
    fn limit_vel(&self, vel: Vec2) -> Vec2 {
        let speed = vel.length();
        if speed > self.max_speed {
            return vel.normalize() * self.max_speed;
        } else if speed < self.min_speed {
            return vel.normalize() * self.min_speed;
        }
        vel
    }
    pub fn tick(&mut self) {
        for i in 0..self.boids.len() {
            // calculate updated velocity
            let sep = self.separation(i);
            let align = self.alignment(i);
            let cohesion = self.cohesion(i);
            let turn = self.handle_margins(i);
            self.boids[i].vel += sep + align + cohesion + turn;

            // constrain velocity
            self.boids[i].vel = self.limit_vel(self.boids[i].vel);

            // update position
            let vel = self.boids[i].vel;
            self.boids[i].pos += vel;
        }
    }
}
