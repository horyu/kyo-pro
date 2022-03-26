#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        ab: (usize, usize),
        cd: (usize, usize),
    };
    let tf = ab.0 < cd.0 || (ab.0 == cd.0 && ab.1 <= cd.1);
    println!("{}", ["Aoki", "Takahashi"][tf as usize]);
}
