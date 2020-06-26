#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: f64
    };
    let n = (n % 12) as f64;
    let short = (360.0 / 12.0) * (n + m / 60.0);
    let long = (360.0 / 60.0) * m;
    let mut abs = (short - long).abs();
    if abs > 180.0 {
        abs = 360.0 - abs;
    }
    println!("{}", abs);
}
