#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        ee: [f64; q],
    };
    for e in ee {
        let theta = e * 2.0 / t * std::f64::consts::PI;
        let pos = (0.0, -l / 2.0 * theta.sin(), l / 2.0 * (1.0 - theta.cos()));
        let xy = ((x - pos.0).powi(2) + (y - pos.1).powi(2)).sqrt();
        let tan = pos.2 / xy;
        let rs = tan.atan() * 180.0 / std::f64::consts::PI;
        println!("{rs}");
    }
}
