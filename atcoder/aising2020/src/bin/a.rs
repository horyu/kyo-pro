#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize
    };
    let mut count = 0;
    for i in l..=r {
        count += (i % d == 0) as i32;
    }
    println!("{}", count);
}
