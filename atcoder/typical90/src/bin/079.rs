#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut aaa: [[isize; w]; h],
        mut bbb: [[isize; w]; h],
    };
    let mut rs = 0;
    for x in 0..(h - 1) {
        for y in 0..(w - 1) {
            let diff = bbb[x][y] - aaa[x][y];
            rs += diff.abs();
            for (dx, dy) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                aaa[x + dx][y + dy] += diff;
            }
        }
    }
    if aaa == bbb {
        println!("Yes\n{rs}");
    } else {
        println!("No");
    }
}
