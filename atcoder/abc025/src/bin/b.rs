#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: isize,
        b: isize,
        ssdd: [(String, isize); n]
    };
    let mut pos = 0;
    for (s, d) in ssdd {
        let diff = match d {
            d if d < a => a,
            d if d > b => b,
            _ => d,
        };
        if s == "West" {
            pos -= diff;
        } else {
            pos += diff;
        }
    }
    match pos.cmp(&0) {
        Ordering::Greater => println!("East {}", pos),
        Ordering::Less => println!("West {}", -pos),
        Ordering::Equal => println!("0"),
    }
}
