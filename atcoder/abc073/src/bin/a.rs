#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: String
    };
    println!("{}", ["No", "Yes"][n.contains("9") as usize]);
}
