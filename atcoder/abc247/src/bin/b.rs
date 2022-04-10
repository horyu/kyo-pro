#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        sstt: [(String, String); n]
    };
    for i in 0..n {
        let (s, t) = &sstt[i];
        let mut stf = false;
        let mut ttf = false;
        for j in 0..n {
            if i == j {
                continue;
            }
            stf |= s == &sstt[j].0 || s == &sstt[j].1;
            ttf |= t == &sstt[j].0 || t == &sstt[j].1;
        }
        if stf && ttf {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
