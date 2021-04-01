#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let all_xor = aa.iter().fold(0, |acc, &a| acc ^ a);
    for a in aa {
        println!("{}", all_xor ^ a);
    }
}
