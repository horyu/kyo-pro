#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    };
    println!("{}", ["A", "B"][((x - a).abs() > (x - b).abs()) as usize]);
}
