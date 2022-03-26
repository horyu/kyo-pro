#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        ab: [usize; 2],
        cd: [usize; 2],
    };
    let tf = ab <= cd;
    println!("{}", ["Aoki", "Takahashi"][tf as usize]);
}
