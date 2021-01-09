#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    let rs = match s.as_str() {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        "Rainy" => "Sunny",
        _ => unreachable!(),
    };
    println!("{}", rs);
}
