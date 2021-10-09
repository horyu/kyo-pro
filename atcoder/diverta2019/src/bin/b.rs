#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        r: usize,
        g: usize,
        h: usize,
        n: usize
    };
    let mut cnt = 0;
    for i in 0..=(n / r) {
        let n = n - r * i;
        for j in 0..=(n / g) {
            if (n - g * j) % h == 0 {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
