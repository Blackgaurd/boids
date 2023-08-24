use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

use crate::vec2::{Vec2, Vec2Array};

#[derive(Debug)]
#[wasm_bindgen]
pub struct QuadTreeNode {
    tl_idx: usize,
    bl_idx: usize,
    tr_idx: usize,
    br_idx: usize,
    pub rect_tl: Vec2,
    pub rect_dims: Vec2,
    points: Vec<Vec2>,
}
impl QuadTreeNode {
    pub fn get_points(&self) -> &Vec<Vec2> {
        &self.points
    }
}
#[wasm_bindgen]
impl QuadTreeNode {
    pub fn new(rect_top_left: Vec2, rect_dims: Vec2) -> Self {
        Self {
            tl_idx: 0,
            bl_idx: 0,
            tr_idx: 0,
            br_idx: 0,
            rect_tl: rect_top_left,
            rect_dims,
            points: Vec::new(),
        }
    }
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(point);
    }
    pub fn num_points(&self) -> usize {
        self.points.len()
    }
    pub fn children(&self) -> Vec<usize> {
        vec![self.tl_idx, self.tr_idx, self.bl_idx, self.br_idx]
    }
}

/**
   returns whether or not a
   point is inside a square
*/
fn in_square(point: Vec2, top_left: Vec2, width: f64) -> bool {
    top_left.x <= point.x
        && point.x <= top_left.x + width
        && top_left.y <= point.y
        && point.y <= top_left.y + width
}
#[test]
fn test_in_square() {
    assert!(in_square(Vec2::new(1.0, 2.0), Vec2::zero(), 5.0));
    assert!(in_square(Vec2::zero(), Vec2::zero(), 1.0));
    assert!(in_square(Vec2::new(5.0, 6.0), Vec2::new(5.0, 3.0), 8.3));
    assert!(!in_square(Vec2::new(10.0, 4.0), Vec2::new(101.0, 4.0), 7.0));
}

/**
  returns whether or not a
  point is inside a rectangle
*/
fn in_rect(point: Vec2, top_left: Vec2, dims: Vec2) -> bool {
    top_left.x <= point.x
        && point.x <= top_left.x + dims.x
        && top_left.y <= point.y
        && point.y <= top_left.y + dims.y
}

/**
   returns whether or not
   two rectangles intersect
*/
fn isect_rects(top_left1: Vec2, dims1: Vec2, top_left2: Vec2, dims2: Vec2) -> bool {
    // TODO: find a more efficient method
    in_rect(top_left1, top_left2, dims2)
        || in_rect(top_left1 + dims1.keep_x(), top_left2, dims2)
        || in_rect(top_left1 + dims1.keep_y(), top_left2, dims2)
        || in_rect(top_left1 + dims1, top_left2, dims2)
        || in_rect(top_left2, top_left1, dims1)
        || in_rect(top_left2 + dims2.keep_x(), top_left1, dims1)
        || in_rect(top_left2 + dims2.keep_y(), top_left1, dims1)
        || in_rect(top_left2 + dims2, top_left1, dims1)
        || (top_left1.x <= top_left2.x
            && top_left2.x + dims2.x <= top_left1.x + dims1.x
            && top_left2.y <= top_left1.y
            && top_left1.y + dims1.y <= top_left2.y + dims2.y)
        || (top_left2.x <= top_left1.x
            && top_left1.x + dims1.x <= top_left2.x + dims2.x
            && top_left1.y <= top_left2.y
            && top_left2.y + dims2.y <= top_left1.y + dims1.y)
}

#[wasm_bindgen]
pub struct QuadTree {
    // root is at nodes[0]
    pub dims: Vec2,
    nodes: Vec<QuadTreeNode>,
    max_points: usize,
    pub num_points: u32,
}
impl QuadTree {
    pub fn get_nodes(&self) -> &Vec<QuadTreeNode> {
        &self.nodes
    }
    pub fn query_square(&self, top_left: Vec2, width: f64) -> Vec<Vec2> {
        let mut ret: Vec<Vec2> = Vec::new();
        let mut q: VecDeque<usize> = [0].into();
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            for &point in self.nodes[cur].points.iter() {
                if in_square(point, top_left, width) {
                    ret.push(point);
                }
            }

            // if square intersects any of the four
            // child nodes, repeat loop with that child
            for child_idx in self.nodes[cur].children().iter() {
                if *child_idx == 0 {
                    continue;
                }
                if isect_rects(
                    self.nodes[*child_idx].rect_tl,
                    self.nodes[*child_idx].rect_dims,
                    top_left,
                    Vec2::from(width),
                ) {
                    q.push_back(*child_idx);
                }
            }
        }
        ret
    }
    pub fn query_circle(&self, center: Vec2, radius: f64) -> Vec<Vec2> {
        let square_points = self.query_square(center - Vec2::from(radius), radius * 2.0);
        let mut ret = Vec::new();
        for point in square_points.iter() {
            if point.distance(&center) <= radius {
                ret.push(*point);
            }
        }
        ret
    }
    pub fn query_circle_brute_force(&self, center: Vec2, radius: f64) -> Vec<Vec2> {
        let mut ret = Vec::new();
        for node in self.nodes.iter() {
            for point in node.points.iter() {
                if point.distance(&center) <= radius {
                    ret.push(*point);
                }
            }
        }
        ret
    }
}
#[wasm_bindgen]
impl QuadTree {
    pub fn new(dims: Vec2) -> Self {
        Self {
            dims,
            nodes: vec![QuadTreeNode::new(Vec2::zero(), dims)],
            max_points: 4,
            num_points: 0,
        }
    }

    // for quadtree demo page
    pub fn node_top_left(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].tl_idx
    }
    pub fn node_top_right(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].tr_idx
    }
    pub fn node_bot_left(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].bl_idx
    }
    pub fn node_bot_right(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].br_idx
    }
    pub fn node_num_points(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].num_points()
    }
    pub fn get_node_point(&self, node_idx: usize, point_idx: usize) -> Vec2 {
        self.nodes[node_idx].points[point_idx]
    }
    pub fn wasm_query_circle(&self, center: &Vec2, radius: f64) -> Vec2Array {
        Vec2Array::new(self.query_circle(*center, radius))
    }

    pub fn max_points(&self) -> usize {
        self.max_points
    }
    pub fn add_point(&mut self, point: &Vec2) -> bool {
        if point.x < 0.0 || point.x > self.dims.x {
            return false;
        }
        if point.y < 0.0 || point.y > self.dims.y {
            return false;
        }

        // values for root node
        let mut cur_idx = 0;
        let mut top_left = Vec2::zero();
        let mut bot_right = self.dims;
        loop {
            if self.nodes[cur_idx].num_points() < self.max_points {
                self.nodes[cur_idx].add_point(*point);
                self.num_points += 1;
                return true;
            }

            // x is left/right
            // y is top/bottom
            let mid = (top_left + bot_right) / 2.0;
            let top: bool;
            let left: bool;
            if point.x <= mid.x {
                left = true;
                bot_right.x = mid.x;
            } else {
                left = false;
                top_left.x = mid.x;
            }
            if point.y <= mid.y {
                top = true;
                bot_right.y = mid.y;
            } else {
                top = false;
                top_left.y = mid.y;
            }

            // theres probably a more
            // elegant way to do this
            let half_dims = self.nodes[cur_idx].rect_dims / 2.0;
            if top {
                if left {
                    // insert into top left quadrant
                    if self.nodes[cur_idx].tl_idx == 0 {
                        self.nodes[cur_idx].tl_idx = self.nodes.len();
                        self.nodes
                            .push(QuadTreeNode::new(self.nodes[cur_idx].rect_tl, half_dims));
                    }
                    cur_idx = self.nodes[cur_idx].tl_idx;
                } else {
                    // insert into top right quadrant
                    if self.nodes[cur_idx].tr_idx == 0 {
                        self.nodes[cur_idx].tr_idx = self.nodes.len();
                        self.nodes.push(QuadTreeNode::new(
                            self.nodes[cur_idx].rect_tl + half_dims.keep_x(),
                            half_dims,
                        ));
                    }
                    cur_idx = self.nodes[cur_idx].tr_idx;
                }
            } else {
                if left {
                    // insert into bottom left quadrant
                    if self.nodes[cur_idx].bl_idx == 0 {
                        self.nodes[cur_idx].bl_idx = self.nodes.len();
                        self.nodes.push(QuadTreeNode::new(
                            self.nodes[cur_idx].rect_tl + half_dims.keep_y(),
                            half_dims,
                        ));
                    }
                    cur_idx = self.nodes[cur_idx].bl_idx;
                } else {
                    // insert into bottom right quadrant
                    if self.nodes[cur_idx].br_idx == 0 {
                        self.nodes[cur_idx].br_idx = self.nodes.len();
                        self.nodes.push(QuadTreeNode::new(
                            self.nodes[cur_idx].rect_tl + half_dims,
                            half_dims,
                        ));
                    }
                    cur_idx = self.nodes[cur_idx].br_idx;
                }
            }
        }
    }
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.nodes.push(QuadTreeNode::new(Vec2::zero(), self.dims));
        self.num_points = 0;
    }
}

#[test]
fn test_quadtree_new() {
    let mut tree = QuadTree::new(Vec2::from(123.0));
    tree.add_point(&Vec2::new(12.0, 32.9));
    for child in tree.nodes[0].children().iter() {
        assert_eq!(*child, 0);
    }
}

#[test]
fn test_quadtree_add_point_in_bound() {
    let mut tree = QuadTree::new(Vec2::new(10.0, 10.0));
    for i in 0..=10 {
        for j in 0..=10 {
            assert!(tree.add_point(&Vec2::new(i as f64, j as f64)));
        }
    }
}

#[test]
fn test_quadtree_add_point_out_bound() {
    let mut tree = QuadTree::new(Vec2::from(50.0));
    assert!(!tree.add_point(&Vec2::from(-1.0)));
    assert!(!tree.add_point(&Vec2::new(7.0, 50.3)));
    assert!(!tree.add_point(&Vec2::new(9.0, -12.3)));
    assert!(!tree.add_point(&Vec2::new(98.3, 9.3)));
    assert!(!tree.add_point(&Vec2::new(-12.3, 34.2)));
    assert!(!tree.add_point(&Vec2::new(99.9, 88.2)));
}

#[test]
fn test_quadtree_child_nodes() {
    let mut tree = QuadTree::new(Vec2::from(10.0));
    // fill top layer
    for _ in 0..tree.max_points() {
        tree.add_point(&Vec2::from(0.1));
    }
    // insert into top left
    tree.add_point(&Vec2::new(3.9, 4.5));
    // insert into bottom right
    tree.add_point(&Vec2::new(9.4, 6.1));
    // insert into bottom left
    tree.add_point(&Vec2::new(0.9, 9.6));
    // insert into top right
    tree.add_point(&Vec2::new(7.7, 2.5));

    let nodes = tree.get_nodes();
    assert_eq!(nodes[1].get_points()[0], Vec2::new(3.9, 4.5));
    assert_eq!(nodes[2].get_points()[0], Vec2::new(9.4, 6.1));
    assert_eq!(nodes[3].get_points()[0], Vec2::new(0.9, 9.6));
    assert_eq!(nodes[4].get_points()[0], Vec2::new(7.7, 2.5));

    // check number of points in each quadrant
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes[0].get_points().len(), 4);
    assert_eq!(nodes[1].get_points().len(), 1);
    assert_eq!(nodes[2].get_points().len(), 1);
    assert_eq!(nodes[3].get_points().len(), 1);
    assert_eq!(nodes[4].get_points().len(), 1);

    // check the size of each quadrant
    // root
    assert_eq!(nodes[0].rect_tl, Vec2::zero());
    assert_eq!(nodes[0].rect_dims, Vec2::from(10.0));
    // top left
    assert_eq!(nodes[1].rect_tl, Vec2::zero());
    assert_eq!(nodes[1].rect_dims, Vec2::from(5.0));
    // bottom right
    assert_eq!(nodes[2].rect_tl, Vec2::from(5.0));
    assert_eq!(nodes[2].rect_dims, Vec2::from(5.0));
    // bottom left
    assert_eq!(nodes[3].rect_tl, Vec2::new(0.0, 5.0));
    assert_eq!(nodes[3].rect_dims, Vec2::from(5.0));
    // top left
    assert_eq!(nodes[4].rect_tl, Vec2::new(5.0, 0.0));
    assert_eq!(nodes[4].rect_dims, Vec2::from(5.0));
}

#[test]
fn test_quadtree_query_square() {
    let mut tree = QuadTree::new(Vec2::from(10.0));
    for i in 0..10 {
        for j in 0..10 {
            tree.add_point(&Vec2::new(i as f64, j as f64));
        }
    }
    assert_eq!(tree.query_square(Vec2::zero(), tree.dims.x).len(), 10 * 10);
    assert_eq!(
        tree.query_square(Vec2::zero(), tree.dims.x / 2.0).len(),
        6 * 6
    );
    assert_eq!(tree.query_square(Vec2::new(0.5, 6.7), 3.4).len(), 3 * 3);
}

#[test]
fn test_quadtree_query_circle() {
    let mut tree = QuadTree::new(Vec2::from(10.0));
    for i in 0..=10 {
        for j in 0..=10 {
            tree.add_point(&Vec2::new(i as f64, j as f64));
        }
    }
    assert_eq!(
        tree.query_circle(Vec2::from(5.0), 5.0).len(),
        tree.query_circle_brute_force(Vec2::from(5.0), 5.0).len()
    );
    assert_eq!(
        tree.query_circle(Vec2::zero(), 3.5).len(),
        tree.query_circle_brute_force(Vec2::zero(), 3.5).len()
    );
    assert_eq!(
        tree.query_circle(Vec2::new(83.2, 45.6), 22.2).len(),
        tree.query_circle_brute_force(Vec2::new(83.2, 45.6), 22.2)
            .len()
    );

    // weird edge case
    // numbers from randomly clicking points
    let mut tree = QuadTree::new(Vec2::new(1352.0, 342.0));
    tree.add_point(&Vec2::new(154.0, 28.0));
    tree.add_point(&Vec2::new(227.0, 28.0));
    tree.add_point(&Vec2::new(188.0, 45.0));
    tree.add_point(&Vec2::new(186.0, 114.0));
    tree.add_point(&Vec2::new(838.0, 71.0));
    tree.add_point(&Vec2::new(906.0, 71.0));
    tree.add_point(&Vec2::new(917.0, 62.0));
    tree.add_point(&Vec2::new(889.0, 96.0));
    assert!(!tree
        .query_square(Vec2::new(873.0, 77.0) - Vec2::from(145.0), 145.0 * 2.0)
        .is_empty());
    assert!(!tree.query_circle(Vec2::new(873.0, 77.0), 145.0).is_empty());
    assert_eq!(
        tree.query_circle(Vec2::new(873.0, 77.0), 145.0).len(),
        tree.query_circle_brute_force(Vec2::new(873.0, 77.0), 145.0)
            .len()
    );
}
