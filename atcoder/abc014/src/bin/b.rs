#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut x: usize,
        aa: [usize; n]
    };
    let mut sum = 0;
    for i in 0..n {
        if x % 2 == 1 {
            sum += aa[i];
        }
        x /= 2;
    }
    println!("{}", sum);
}
