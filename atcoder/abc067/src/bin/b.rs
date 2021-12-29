#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ll: [usize; n]
    };
    ll.sort_unstable();
    ll.reverse();
    println!("{}", ll[0..k].iter().sum::<usize>());
}
