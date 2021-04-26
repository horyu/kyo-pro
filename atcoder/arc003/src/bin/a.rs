#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        rr: Chars
    };
    let sum = rr.into_iter().fold(0usize, |acc, r| {
        acc + match r {
            'A' => 4,
            'B' => 3,
            'C' => 2,
            'D' => 1,
            _ => 0,
        }
    });
    println!("{}", sum as f64 / n as f64);
}
