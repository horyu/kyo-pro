#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        v: isize,
        b: isize,
        w: isize,
        t: isize,
    };
    let tf = if v <= w {
        false
    } else {
        (b - a).abs() <= t * (w - v).abs()
    };
    println!("{}", ["NO", "YES"][tf as usize]);
}
