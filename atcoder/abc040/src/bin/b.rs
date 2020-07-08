#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize
    };
    let mut min = std::isize::MAX;
    for tate in 1..=n {
        let yoko = n / tate;
        let amari = n - tate * yoko;
        min = std::cmp::min(min, (tate - yoko).abs() + amari)
    }
    println!("{}", min);
}
