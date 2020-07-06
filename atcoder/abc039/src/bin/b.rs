#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: usize
    };
    for i in 1usize.. {
        if i.pow(4) == x {
            println!("{}", i);
            return;
        }
    }
}
