#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    };
    let x = d * std::f64::consts::PI / 180.0;
    let aa = x.cos() * a - x.sin() * b;
    let bb = x.sin() * a + x.cos() * b;
    println!("{aa} {bb}");
}
