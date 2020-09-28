#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n]
    };
    aa.sort();
    aa.reverse();
    let mut num = 0;
    for (i, a) in aa.iter().enumerate() {
        num += (-1isize).pow(i as u32) * a;
    }
    println!("{}", num);
}
