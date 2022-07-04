#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

use crate::lazy_segment_tree::LazySegmentTree;

fn main() {
    input! {
        w: usize,
        n: usize,
        llrr: [(usize, usize); n],
    };
    let mut lst = LazySegmentTree::new(
        w,
        || 0usize,
        |&s, &t| s.max(t),
        |&f: &usize, &x: &usize| f.max(x),
        |&f: &usize, &g: &usize| f.max(g),
        || 0usize,
    );
    for (l, r) in llrr {
        let hh = lst.prod((l - 1)..r) + 1;
        lst.apply_range((l - 1)..r, hh);

        println!("{hh}");
    }
}

// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/data_structure/lazy_segment_tree.rs
pub mod lazy_segment_tree {
    type Range = std::ops::Range<usize>;

    pub struct LazySegmentTree<S, Op, E, F, Mapping, Composition, Id> {
        n: usize,
        size: usize,
        log: usize,
        data: Vec<S>,
        lazy: Vec<F>,
        op: Op,
        e: E,
        mapping: Mapping,
        composition: Composition,
        id: Id,
    }

    impl<S, Op, E, F, Mapping, Composition, Id> LazySegmentTree<S, Op, E, F, Mapping, Composition, Id>
    where
        S: Clone,
        E: Fn() -> S,
        F: Clone,
        Op: Fn(&S, &S) -> S,
        Mapping: Fn(&F, &S) -> S,
        Composition: Fn(&F, &F) -> F,
        Id: Fn() -> F,
    {
        pub fn new(
            n: usize,
            e: E,
            op: Op,
            mapping: Mapping,
            composition: Composition,
            id: Id,
        ) -> Self {
            let size = n.next_power_of_two() as usize;
            LazySegmentTree {
                n,
                size,
                log: size.trailing_zeros() as usize,
                data: vec![e(); 2 * size],
                lazy: vec![id(); size],
                e,
                op,
                mapping,
                composition,
                id,
            }
        }
        pub fn set(&mut self, mut index: usize, value: S) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = value;
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }

        pub fn get(&mut self, mut index: usize) -> S {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index].clone()
        }

        pub fn prod(&mut self, range: Range) -> S {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l < r && r <= self.n);

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }

            let mut sum_l = (self.e)();
            let mut sum_r = (self.e)();
            while l < r {
                if l & 1 != 0 {
                    sum_l = (self.op)(&sum_l, &self.data[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    sum_r = (self.op)(&self.data[r], &sum_r);
                }
                l >>= 1;
                r >>= 1;
            }

            (self.op)(&sum_l, &sum_r)
        }

        pub fn all_prod(&self) -> S {
            self.data[1].clone()
        }

        pub fn apply(&mut self, mut index: usize, f: F) {
            assert!(index < self.n);
            index += self.size;
            for i in (1..=self.log).rev() {
                self.push(index >> i);
            }
            self.data[index] = (self.mapping)(&f, &self.data[index]);
            for i in 1..=self.log {
                self.update(index >> i);
            }
        }
        pub fn apply_range(&mut self, range: Range, f: F) {
            let mut l = range.start;
            let mut r = range.end;
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }

            l += self.size;
            r += self.size;

            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }

            {
                let mut l = l;
                let mut r = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }

            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }

        fn update(&mut self, k: usize) {
            self.data[k] = (self.op)(&self.data[2 * k], &self.data[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F) {
            self.data[k] = (self.mapping)(&f, &self.data[k]);
            if k < self.size {
                self.lazy[k] = (self.composition)(&f, &self.lazy[k]);
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lazy[k].clone());
            self.all_apply(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = (self.id)();
        }
    }
}
