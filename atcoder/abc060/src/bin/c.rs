#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        t: usize,
        mut tt: [usize; n]
    };
    tt.push(std::usize::MAX);
    let mut sum = 0;
    for (&a, &b) in tt.iter().tuple_windows() {
        sum += if a + t < b { t } else { b - a };
    }
    println!("{}", sum);
}
