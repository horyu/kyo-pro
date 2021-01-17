#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: i32
    };
    // (1..9).to_a.repeated_combination(2).map{|xx| xx[0] * xx[1]}.uniq.join(", ")
    let arr = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18, 15, 21, 24, 27, 20, 28, 32, 36, 25, 30, 35,
        40, 45, 42, 48, 54, 49, 56, 63, 64, 72, 81,
    ];
    println!("{}", ["No", "Yes"][arr.contains(&n) as usize]);
}
