#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    s.sort();
    println!("{}", ["No", "Yes"][(s == vec!['a', 'b', 'c']) as usize]);
}
