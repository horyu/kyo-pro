#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: u8,
        y: u8
    };
    println!("{}", if y > x { "Better" } else { "Worse" });
}
