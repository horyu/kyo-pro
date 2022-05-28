#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let tf = (a <= b && b <= c) || c <= b && b <= a;
    println!("{}", ["No", "Yes"][tf as usize]);
}
