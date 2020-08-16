#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: i128,
        k: i128,
        d: i128
    };
    let x = x.abs();
    if k * d <= x {
        println!("{}", x - k * d);
    } else {
        let pos = if (k - x / d) % 2 == 0 {
            x % d
        } else {
            (x % d - d).abs()
        };
        println!("{}", pos);
    }
}
