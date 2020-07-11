#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut count = 0;
    for i in (0..n).step_by(2) {
        count += (aa[i] % 2 == 1) as i32;
    }
    println!("{}", count);
}
