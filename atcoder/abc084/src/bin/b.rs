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
        match s.pop().unwrap() {
            '-' => {
                println!("No");
                return;
            }
            _ => (),
        }
    }
    match s.pop().unwrap() {
        '-' => (),
        _ => {
            println!("No");
            return;
        }
    }
    for _ in 0..a {
        match s.pop().unwrap() {
            '-' => {
                println!("No");
                return;
            }
            _ => (),
        }
    }
    println!("Yes");
}
