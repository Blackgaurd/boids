use js_sys::Int32Array;

use crate::vec2::{Position, Vec2};
use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
pub struct QuadTreeNode<T>
where
    T: Position + Copy,
{
    tl_idx: usize,
    bl_idx: usize,
    tr_idx: usize,
    br_idx: usize,
    pub rect_tl: Vec2,
    pub rect_dims: Vec2,
    items: Vec<T>,
}
impl<T> QuadTreeNode<T>
where
    T: Position + Copy,
{
    pub fn get_items(&self) -> Vec<T> {
        self.items.to_vec()
    }
}
impl<T> QuadTreeNode<T>
where
    T: Position + Copy,
{
    pub fn new(rect_top_left: Vec2, rect_dims: Vec2) -> Self {
        Self {
            tl_idx: 0,
            bl_idx: 0,
            tr_idx: 0,
            br_idx: 0,
            rect_tl: rect_top_left,
            rect_dims,
            items: Vec::new(),
        }
    }
    pub fn push(&mut self, val: &T) {
        self.items.push(*val);
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn children(&self) -> Vec<usize> {
        vec![self.tl_idx, self.tr_idx, self.bl_idx, self.br_idx]
    }
}

pub struct QuadTree<T>
where
    T: Position + Copy,
{
    // root is at nodes[0]
    pub dims: Vec2,
    nodes: Vec<QuadTreeNode<T>>,
    max_points: usize,
    num_items: usize,
}
impl<T> QuadTree<T>
where
    T: Position + Copy,
{
    pub fn new(dims: Vec2) -> Self {
        Self {
            dims,
            nodes: vec![QuadTreeNode::new(Vec2::zero(), dims)],
            max_points: 4,
            num_items: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.num_items
    }
    pub fn max_points(&self) -> usize {
        self.max_points
    }
    pub fn get_nodes(&self) -> Vec<QuadTreeNode<T>> {
        self.nodes.to_vec()
    }
    pub fn query_square(&self, top_left: Vec2, width: f64) -> Vec<T> {
        let mut ret = Vec::new();
        let mut q: VecDeque<usize> = [0].into();
        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            for item in self.nodes[cur].items.iter() {
                if in_square(item.pos(), top_left, width) {
                    ret.push(*item);
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
    pub fn query_circle(&self, center: Vec2, radius: f64) -> Vec<T> {
        let in_square = self.query_square(center - Vec2::from(radius), radius * 2.0);
        let mut ret = Vec::new();
        for item in in_square.iter() {
            if item.pos().distance(&center) <= radius {
                ret.push(*item);
            }
        }
        ret
    }
    pub fn query_circle_brute_force(&self, center: Vec2, radius: f64) -> Vec<T> {
        let mut ret = Vec::new();
        for node in self.nodes.iter() {
            for item in node.items.iter() {
                if item.pos().distance(&center) <= radius {
                    ret.push(*item);
                }
            }
        }
        ret
    }
    pub fn push(&mut self, item: &T) -> bool {
        if item.x() < 0.0 || item.x() > self.dims.x {
            return false;
        }
        if item.y() < 0.0 || item.y() > self.dims.y {
            return false;
        }

        // values for root node
        let mut cur_idx = 0;
        let mut top_left = Vec2::zero();
        let mut bot_right = self.dims;
        loop {
            if self.nodes[cur_idx].len() < self.max_points {
                self.nodes[cur_idx].push(item);
                self.num_items += 1;
                return true;
            }

            // x is left/right
            // y is top/bottom
            let mid = (top_left + bot_right) / 2.0;
            let top: bool;
            let left: bool;
            if item.x() <= mid.x {
                left = true;
                bot_right.x = mid.x;
            } else {
                left = false;
                top_left.x = mid.x;
            }
            if item.y() <= mid.y {
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
        self.num_items = 0;
    }

    pub fn node_len(&self, node_idx: usize) -> usize {
        self.nodes[node_idx].len()
    }
    pub fn node_item_pos(&self, node_idx: usize, item_idx: usize) -> Vec2 {
        self.nodes[node_idx].items[item_idx].pos()
    }
    pub fn node_children(&self, node_idx: usize) -> Int32Array {
        let ret = Int32Array::new_with_length(4);
        for (i, child) in self.nodes[node_idx].children().iter().enumerate() {
            ret.set_index(i as u32, *child as i32);
        }
        ret
    }
}

#[test]
fn test_quadtree_new() {
    let mut tree = QuadTree::new(Vec2::from(123.0));
    tree.push(&Vec2::new(12.0, 32.9));
    for child in tree.nodes[0].children().iter() {
        assert_eq!(*child, 0);
    }
}

#[test]
fn test_quadtree_push_in_bound() {
    let mut tree = QuadTree::new(Vec2::new(10.0, 10.0));
    for i in 0..=10 {
        for j in 0..=10 {
            assert!(tree.push(&Vec2::new(i as f64, j as f64)));
        }
    }
}

#[test]
fn test_quadtree_push_out_bound() {
    let mut tree = QuadTree::new(Vec2::from(50.0));
    assert!(!tree.push(&Vec2::from(-1.0)));
    assert!(!tree.push(&Vec2::new(7.0, 50.3)));
    assert!(!tree.push(&Vec2::new(9.0, -12.3)));
    assert!(!tree.push(&Vec2::new(98.3, 9.3)));
    assert!(!tree.push(&Vec2::new(-12.3, 34.2)));
    assert!(!tree.push(&Vec2::new(99.9, 88.2)));
}

#[test]
fn test_quadtree_child_nodes() {
    let mut tree = QuadTree::new(Vec2::from(10.0));
    // fill top layer
    for _ in 0..tree.max_points() {
        tree.push(&Vec2::from(0.1));
    }
    // insert into top left
    tree.push(&Vec2::new(3.9, 4.5));
    // insert into bottom right
    tree.push(&Vec2::new(9.4, 6.1));
    // insert into bottom left
    tree.push(&Vec2::new(0.9, 9.6));
    // insert into top right
    tree.push(&Vec2::new(7.7, 2.5));

    let nodes = tree.get_nodes();
    assert_eq!(nodes[1].get_items()[0], Vec2::new(3.9, 4.5));
    assert_eq!(nodes[2].get_items()[0], Vec2::new(9.4, 6.1));
    assert_eq!(nodes[3].get_items()[0], Vec2::new(0.9, 9.6));
    assert_eq!(nodes[4].get_items()[0], Vec2::new(7.7, 2.5));

    // check number of points in each quadrant
    assert_eq!(nodes.len(), 5);
    assert_eq!(nodes[0].get_items().len(), 4);
    assert_eq!(nodes[1].get_items().len(), 1);
    assert_eq!(nodes[2].get_items().len(), 1);
    assert_eq!(nodes[3].get_items().len(), 1);
    assert_eq!(nodes[4].get_items().len(), 1);

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
            tree.push(&Vec2::new(i as f64, j as f64));
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
            tree.push(&Vec2::new(i as f64, j as f64));
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
    tree.push(&Vec2::new(154.0, 28.0));
    tree.push(&Vec2::new(227.0, 28.0));
    tree.push(&Vec2::new(188.0, 45.0));
    tree.push(&Vec2::new(186.0, 114.0));
    tree.push(&Vec2::new(838.0, 71.0));
    tree.push(&Vec2::new(906.0, 71.0));
    tree.push(&Vec2::new(917.0, 62.0));
    tree.push(&Vec2::new(889.0, 96.0));
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
