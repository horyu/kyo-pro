#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        mut s: Chars
    };
    for _ in 0..b {
        if s.pop().unwrap() == '-' {
            println!("No");
            return;
        }
    }
    if s.pop().unwrap() != '-' {
        println!("No");
        return;
    }
    for _ in 0..a {
        if s.pop().unwrap() == '-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
