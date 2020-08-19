#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ll: [usize; n]
    };
    ll.sort();
    ll.reverse();
    println!("{}", ll[0..k].iter().sum::<usize>());
}
