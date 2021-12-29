#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut ss = s.clone();
    ss.sort_unstable();
    ss.dedup();
    println!("{}", ["no", "yes"][(s.len() == ss.len()) as usize]);
}
