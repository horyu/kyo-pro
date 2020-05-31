#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: u128,
        b: String
    };
    if let [s, t] = *b.split('.').collect::<Vec<_>>() {
        let s: u128 = s.parse().unwrap();
        let t: u128 = t.parse().unwrap();
        println!("{}", a * (100 * s + t) / 100);
    }
}
