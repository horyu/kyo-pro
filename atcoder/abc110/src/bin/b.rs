#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: isize,
        y: isize,
        mut xx: [isize; n],
        mut yy: [isize; m]
    };
    xx.push(x);
    yy.push(y);
    let x_max = xx.iter().max().unwrap();
    let y_min = yy.iter().min().unwrap();
    println!("{}", ["War", "No War"][(x_max < y_min) as usize]);
}
