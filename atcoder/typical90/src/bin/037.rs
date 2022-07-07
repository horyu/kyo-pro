#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        w: usize,
        n: usize,
        llrrvv: [(usize, usize, usize); n],
    };
    let mut st = SegmentTree::new(w + 1, |x: usize, y: usize| x.max(y));
    st.update(0, 0);
    for (l, r, v) in llrrvv {
        for i in (l..=w).rev() {
            if let Some(max) = st.query(i.saturating_sub(r)..(i - l + 1)) {
                let cur = st.query(i..(i + 1)).unwrap_or_default();
                st.update(i, cur.max(max + v));
            }
        }
    }
    if let Some(rs) = st.query(w..(w + 1)) {
        println!("{rs}");
    } else {
        println!("-1");
    }
}

// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/data_structure/segment_tree.rs
/// Segment Tree for range queries
pub struct SegmentTree<T, Op> {
    seg: Vec<Option<T>>,
    n: usize,
    op: Op,
}

impl<T, Op> SegmentTree<T, Op>
where
    T: Copy,
    Op: Fn(T, T) -> T + Copy,
{
    pub fn new(size: usize, op: Op) -> SegmentTree<T, Op> {
        let mut m = size.next_power_of_two();
        if m == size {
            m *= 2;
        }
        SegmentTree {
            seg: vec![None; m * 2],
            n: m,
            op,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = Some(value);
        while k > 0 {
            k = (k - 1) >> 1;
            let left = self.seg[k * 2 + 1];
            let right = self.seg[k * 2 + 2];
            self.seg[k] = Self::op(left, right, self.op);
        }
    }

    /// Get the result in the array of the range
    pub fn query(&self, range: std::ops::Range<usize>) -> Option<T> {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> Option<T> {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            None
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            Self::op(x, y, self.op)
        }
    }

    fn op(a: Option<T>, b: Option<T>, f: Op) -> Option<T> {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => a.or(b),
        }
    }
}
