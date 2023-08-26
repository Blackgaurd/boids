use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    quadtree::QuadTree,
    vec2::{Position, Vec2},
};

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
impl Position for Boid {
    fn pos(&self) -> Vec2 {
        self.pos
    }
}

#[wasm_bindgen]
pub struct World {
    pub dims: Vec2,
    boids: Vec<Boid>,
    quadtree: QuadTree<Boid>,
    pub protect_range: f64,
    pub visible_range: f64,
    pub avoid_factor: f64,
    pub align_factor: f64,
    pub cohesion_factor: f64,
    pub margin: f64,
    pub turn_factor: f64,
    pub max_speed: f64,
    pub min_speed: f64,
}
#[wasm_bindgen]
impl World {
    pub fn new(
        dims: &Vec2,
        visible_range: f64,
        protect_range: f64,
        avoid_factor: f64,
        align_factor: f64,
        cohesion_factor: f64,
        margin: f64,
        turn_factor: f64,
        max_speed: f64,
        min_speed: f64,
    ) -> Self {
        World {
            dims: *dims,
            boids: Vec::new(),
            quadtree: QuadTree::new(*dims),
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
    pub fn add_boid(&mut self, pos: &Vec2, vel: &Vec2) {
        self.boids.push(Boid::new(*pos, *vel));
        self.quadtree.push(&Boid::new(*pos, *vel));
    }
    pub fn num_boids(&self) -> usize {
        debug_assert_eq!(self.boids.len(), self.quadtree.len());
        self.boids.len()
    }
    pub fn get_boid(&self, i: usize) -> Boid {
        // * what if i >= seld.boids.len()?
        self.boids[i]
    }

    fn separation_brute_force(&self, i: usize) -> Vec2 {
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
    fn separation(&self, cur_pos: Vec2, protected_boids: &Vec<Boid>) -> Vec2 {
        // visible boids includes the current boid
        let mut close = Vec2::zero();
        for boid in protected_boids {
            close += cur_pos - boid.pos;
        }
        close * self.avoid_factor
    }
    fn alignment_brute_force(&self, i: usize) -> Vec2 {
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
    fn alignment(&self, cur_vel: Vec2, visible_boids: &Vec<Boid>) -> Vec2 {
        if visible_boids.len() <= 1 {
            return Vec2::zero();
        }
        let mut vel_avg = -cur_vel;
        for boid in visible_boids {
            vel_avg += boid.vel;
        }
        vel_avg /= visible_boids.len() as f64 - 1.0; // -1.0 to exclude current boid
        vel_avg * self.align_factor
    }
    fn cohesion_brute_force(&self, i: usize) -> Vec2 {
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
    fn cohesion(&self, cur_pos: Vec2, visible_boids: &Vec<Boid>) -> Vec2 {
        if visible_boids.len() <= 1 {
            return Vec2::zero();
        }
        let mut pos_avg = -cur_pos;
        for boid in visible_boids {
            pos_avg += boid.pos;
        }
        pos_avg /= visible_boids.len() as f64 - 1.0; // -1.0 to exclude current boid
        (pos_avg - cur_pos) * self.cohesion_factor
    }
    fn handle_margins(&self, cur_pos: Vec2) -> Vec2 {
        Vec2::new(
            if cur_pos.x < self.margin {
                // outside left border
                self.turn_factor
            } else if cur_pos.x > self.dims.x - self.margin {
                // outside right border
                -self.turn_factor
            } else {
                0.0
            },
            if cur_pos.y < self.margin {
                // outside top margin
                self.turn_factor
            } else if cur_pos.y > self.dims.y - self.margin {
                // outside bottom margin
                -self.turn_factor
            } else {
                0.0
            },
        )
    }
    pub fn tick_brute_force(&mut self) {
        for i in 0..self.boids.len() {
            // calculate updated velocity
            let sep = self.separation_brute_force(i);
            let align = self.alignment_brute_force(i);
            let cohesion = self.cohesion_brute_force(i);
            let turn = self.handle_margins(self.boids[i].pos);
            self.boids[i].vel += sep + align + cohesion + turn;

            // constrain velocity
            self.boids[i].vel = self.boids[i]
                .vel
                .clamp_length(self.min_speed, self.max_speed);

            // update position
            let vel = self.boids[i].vel;
            self.boids[i].pos += vel;
        }
    }
    pub fn tick(&mut self) {
        // update the boids
        for i in 0..self.boids.len() {
            let boid = &self.boids[i];
            let visible = self.quadtree.query_circle(boid.pos, self.visible_range);
            let protected = self.quadtree.query_circle(boid.pos, self.protect_range);

            // calculate updated velocity
            let sep = self.separation(boid.pos, &protected);
            let align = self.alignment(boid.vel, &visible);
            let cohesion = self.cohesion(boid.pos, &visible);
            let turn = self.handle_margins(boid.pos);
            let mut vel = boid.vel + sep + align + cohesion + turn;

            // constrain velocity
            vel = vel.clamp_length(self.min_speed, self.max_speed);

            // update boid
            self.boids[i].pos += vel;
            self.boids[i].vel = vel;
        }

        // rebuild the quadtree
        self.quadtree.clear();
        for boid in &self.boids {
            self.quadtree.push(boid);
        }
    }
}

#[test]
fn test_world_separation() {
    let mut world = World::new(
        &Vec2::new(100.0, 100.0),
        15.0,
        15.0,
        1.0,
        1.0,
        1.0,
        0.0,
        0.0,
        10.0,
        0.0,
    );
    for i in (0..100).step_by(10) {
        for j in (0..100).step_by(10) {
            world.add_boid(&Vec2::new(i as f64, j as f64), &Vec2::zero());
        }
    }
    for idx in 0..world.num_boids() {
        let protected = world
            .quadtree
            .query_circle(world.boids[idx].pos, world.protect_range);
        let sep = world.separation(world.boids[idx].pos, &protected);
        let expected = world.separation_brute_force(idx);
        assert_eq!(sep, expected);
    }
}

#[test]
fn test_world_alignment() {
    let mut world = World::new(
        &Vec2::new(100.0, 100.0),
        15.0,
        15.0,
        1.0,
        1.0,
        1.0,
        0.0,
        0.0,
        10.0,
        0.0,
    );
    for i in (0..100).step_by(10) {
        for j in (0..100).step_by(10) {
            world.add_boid(&Vec2::new(i as f64, j as f64), &Vec2::zero());
        }
    }
    for idx in 0..world.num_boids() {
        let visible = world
            .quadtree
            .query_circle(world.boids[idx].pos, world.visible_range);
        let align = world.alignment(world.boids[idx].vel, &visible);
        let expected = world.alignment_brute_force(idx);
        assert_eq!(align, expected);
    }
}

#[test]
fn test_world_cohesion() {
    let mut world = World::new(
        &Vec2::new(100.0, 100.0),
        15.0,
        15.0,
        1.0,
        1.0,
        1.0,
        0.0,
        0.0,
        10.0,
        0.0,
    );
    for i in (0..100).step_by(10) {
        for j in (0..100).step_by(10) {
            world.add_boid(&Vec2::new(i as f64, j as f64), &Vec2::zero());
        }
    }
    for idx in 0..world.num_boids() {
        let visible = world
            .quadtree
            .query_circle(world.boids[idx].pos, world.visible_range);
        let cohesion = world.cohesion(world.boids[idx].pos, &visible);
        let expected = world.cohesion_brute_force(idx);
        assert_eq!(cohesion, expected);
    }
}
