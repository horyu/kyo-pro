#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: isize,
        b: isize,
    };
    let tf = (a - b).abs() == 1 || (a == 1 && b == 10) || (b == 1 && a == 10);
    println!("{}", ["No", "Yes"][tf as usize]);
}
