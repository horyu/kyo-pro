#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        ss: [String; 12]
    };
    let count = ss.iter().fold(0, |i, s| i + (s.contains('r') as i32));
    println!("{}", count);
}
