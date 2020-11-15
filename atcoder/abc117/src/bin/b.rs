#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ll: [usize; n]
    };
    let longest = ll.iter().max().unwrap();
    let tf = ll.iter().sum::<usize>() - longest > *longest;
    println!("{}", ["No", "Yes"][tf as usize]);
}
