#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        tt: [usize; n]
    };
    println!("{}", tt.iter().min().unwrap());
}
