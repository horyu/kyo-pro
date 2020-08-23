#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: Chars
    };
    let sum = n.iter().map(|&c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("{}", ["No", "Yes"][(sum % 9 == 0) as usize]);
}
