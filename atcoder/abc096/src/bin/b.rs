#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut abc: [usize; 3],
        k: usize
    };
    abc.sort();
    for _ in 0..k {
        abc[2] *= 2;
    }
    println!("{}", abc.iter().sum::<usize>());
}
