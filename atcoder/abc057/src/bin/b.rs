#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(isize, isize); n],
        ccdd: [(isize, isize); m],
    };
    for (x1, y1) in aabb {
        let mut min_len = std::isize::MAX;
        let mut pos = 0usize;
        for (index, (x2, y2)) in ccdd.iter().enumerate() {
            let len = (x1 - x2).abs() + (y1 - y2).abs();
            if len < min_len {
                min_len = len;
                pos = index;
            }
        }
        println!("{}", pos + 1);
    }
}
