use std::collections::VecDeque;

use js_sys::Int32Array;
use quadtree::QuadTree;
use vec2::Vec2;
use wasm_bindgen::prelude::wasm_bindgen;

mod quadtree;
mod vec2;
mod world;

/**
Wrapper class around Vec<Vec2> for WASM
 */
#[wasm_bindgen]
pub struct WasmVec2Array {
    arr: Vec<Vec2>,
}
impl WasmVec2Array {
    pub fn new(arr: Vec<Vec2>) -> Self {
        Self { arr }
    }
}
#[wasm_bindgen]
impl WasmVec2Array {
    pub fn len(&self) -> usize {
        self.arr.len()
    }
    pub fn get(&self, idx: usize) -> Vec2 {
        if idx >= self.len() {
            return Vec2::from(-1.0);
        }
        self.arr[idx]
    }
}

/**
Wrapper class around QuadTree<Vec2> for WASM
 */
#[wasm_bindgen]
pub struct WasmQuadTree {
    tree: QuadTree<Vec2>,
}
#[wasm_bindgen]
impl WasmQuadTree {
    pub fn new(dims: &Vec2) -> Self {
        Self {
            tree: QuadTree::new(Vec2::zero(), *dims),
        }
    }
    pub fn len(&self) -> usize {
        self.tree.len()
    }
    pub fn dims(&self) -> Vec2 {
        self.tree.border_dims
    }
    pub fn push(&mut self, point: &Vec2) {
        self.tree.push(point);
    }
    pub fn query_circle(&self, center: &Vec2, radius: f64) -> WasmVec2Array {
        WasmVec2Array::new(self.tree.query_circle(*center, radius))
    }
    pub fn clear(&mut self) {
        self.tree.reset(Vec2::zero(), self.tree.border_dims);
    }
    pub fn node_len(&self, node_idx: usize) -> usize {
        self.tree.node_len(node_idx)
    }
    pub fn node_item_pos(&self, node_idx: usize, item_idx: usize) -> Vec2 {
        self.tree.node_item_pos(node_idx, item_idx)
    }
    pub fn node_children(&self, node_idx: usize) -> Int32Array {
        self.tree.node_children(node_idx)
    }
}

#[wasm_bindgen]
pub struct RollingAverage {
    q: VecDeque<f64>,
    sum: f64,
    max_len: usize,
}
#[wasm_bindgen]
impl RollingAverage {
    pub fn new(max_len: usize) -> Self {
        Self {
            q: VecDeque::with_capacity(max_len),
            sum: 0.0,
            max_len,
        }
    }
    pub fn push(&mut self, val: f64) {
        if self.q.len() == self.max_len {
            self.sum -= self.q.pop_front().unwrap();
        }
        self.q.push_back(val);
        self.sum += val;
    }
    pub fn query(&self) -> f64 {
        if self.q.is_empty() {
            0.0
        } else {
            self.sum / self.q.len() as f64
        }
    }
}

#[test]
fn test_rolling_avg() {
    let mut avg = RollingAverage::new(3);
    assert_eq!(avg.query(), 0.0);
    avg.push(1.0);
    assert_eq!(avg.query(), 1.0);
    avg.push(2.0);
    assert_eq!(avg.query(), 1.5);
    avg.push(3.0);
    assert_eq!(avg.query(), 2.0);
    avg.push(4.0);
    assert_eq!(avg.query(), 3.0);
}
