#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut s: Chars
    };
    s.sort_unstable();
    let rs: String = s.into_iter().collect();
    println!("{rs}");
}
