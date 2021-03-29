#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    };
    use std::f64::consts::PI;
    let a_theta = (30.0 * h + m / 2.0) * PI / 180.0;
    let b_theta = (6.0 * m) * PI / 180.0;
    println!(
        "{}",
        (a.powi(2) + b.powi(2) - 2.0 * a * b * (b_theta - a_theta).cos()).sqrt()
    );
}
