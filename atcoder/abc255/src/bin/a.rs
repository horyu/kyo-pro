#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        r: Usize1,
        c: Usize1,
        aaa: [[usize; 2]; 2]
    };
    println!("{}", aaa[r][c]);
}
