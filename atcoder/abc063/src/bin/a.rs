#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: u8,
        b: u8
    };
    let sum = a + b;
    if sum >= 10 {
        println!("error");
    } else {
        println!("{}", sum);
    }
}
