#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize
    };
    let s = if b <= a {
        "delicious"
    } else if b <= a + x {
        "safe"
    } else {
        "dangerous"
    };
    println!("{}", s);
}
