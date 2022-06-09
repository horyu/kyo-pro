#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aa: [usize; n],
        ttxxyy: [(u8, usize, usize); q],
    };
    let mut zure = 0usize;
    for (t, x, y) in ttxxyy {
        match t {
            1 => {
                aa.swap((x - 1 + n - zure) % n, (y - 1 + n - zure) % n);
            }
            2 => {
                zure = (zure + 1) % n;
            }
            _ => {
                println!("{}", aa[(x - 1 + n - zure) % n]);
            }
        }
    }
}
