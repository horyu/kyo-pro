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
        s: Chars,
        ttxx: [(usize, usize); q],
    };
    let mut rotate = 0usize;
    for (t, x) in ttxx {
        if t == 1 {
            rotate += x;
            rotate %= n;
        } else {
            println!("{}", s[(n + x - 1 - rotate) % n]);
        }
    }
}
