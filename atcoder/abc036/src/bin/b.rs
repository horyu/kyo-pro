#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        sss: [Chars; n]
    };
    for i in 0..n {
        for j in 0..n {
            print!("{}", sss[n - 1 - j][i]);
        }
        println!();
    }
}
