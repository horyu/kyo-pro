#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: i8,
        b: i8,
        c: i8
    };
    println!("{}", ["No", "Yes"][((a <= c) && (c <= b)) as usize]);
}
