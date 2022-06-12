#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        aabb: [(usize, usize); n],
    };
    let mut bh = BinaryHeap::new();
    for (a, b) in aabb {
        bh.push((b, Some(a)));
    }
    let mut rs = 0;
    for _ in 0..k {
        if let Some((score, aopt)) = bh.pop() {
            rs += score;
            if let Some(a) = aopt {
                bh.push((a - score, None));
            }
        }
    }
    println!("{rs}");
}
