#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        xxx: [usize; 3]
    };
    println!("{}", xxx[0] * xxx[1] / 2);
}
