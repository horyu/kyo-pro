#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: u8,
        b: u8
    };
    let vecs = [vec![1, 3, 5, 7, 8, 10, 12], vec![4, 6, 9, 11], vec![2]];
    let tf = vecs.iter().any(|v| v.contains(&a) && v.contains(&b));
    println!("{}", ["No", "Yes"][tf as usize]);
}
