#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    if n == 1 {
        println!("0");
        return;
    }
    let mut sum = 0;
    let mut pre_height = aa[0];
    for &a in &aa[1..n] {
        if a < pre_height {
            let diff = pre_height - a;
            sum += diff;
        } else {
            pre_height = a
        }
    }
    println!("{}", sum);
}
