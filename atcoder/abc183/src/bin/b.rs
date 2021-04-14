#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    };
    let sy = -sy;
    // y = (gy - sy)/(gx - sx) (x - sx) + sy
    // -sy = (gy - sy)/(gx - sx) (x - sx)
    // -sy / ((gy - sy)/(gx - sx)) + sx = x
    println!("{}", sx - sy * (gx - sx) / (gy - sy));
}
