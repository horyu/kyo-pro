#![allow(unused_imports)]
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, OneElement};
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    match aa.iter().minmax() {
        OneElement(_) => println!("0"),
        MinMax(left, right) => println!("{}", right - left),
        _ => unreachable!(),
    };
}
