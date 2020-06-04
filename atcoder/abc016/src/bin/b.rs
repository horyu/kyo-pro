#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    let wa = (a + b) == c;
    let sa = (a - b) == c;
    println!(
        "{}",
        match (wa, sa) {
            (true, true) => "?",
            (true, false) => "+",
            (false, true) => "-",
            (false, false) => "!",
        }
    );
}
