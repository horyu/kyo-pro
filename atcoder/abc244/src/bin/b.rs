#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        t: Chars
    };
    let mut xy = (0, 0);
    let mut muki = 0;
    for c in t {
        if c == 'S' {
            let (xx, yy) = match muki % 4 {
                0 => (1, 0),
                1 => (0, -1),
                2 => (-1, 0),
                _ => (0, 1),
            };
            xy = (xy.0 + xx, xy.1 + yy);
        } else {
            muki += 1;
        }
    }
    println!("{} {}", xy.0, xy.1);
}
