#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    // sqrt(a) + sqrt(b) < sqrt(c)
    // a + b + 2*sqrt(a*b) < c
    // 4*a*b < (c - a - b)**2
    let tf = if a + b < c {
        4 * a * b < (c - a - b).pow(2u32)
    } else {
        false
    };
    println!("{}", ["No", "Yes"][tf as usize]);
}
