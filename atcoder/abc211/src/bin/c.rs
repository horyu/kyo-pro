#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    const MOD: usize = 1_000_000_007;
    let mut v = vec![0; 8];
    for c in s {
        let n = match c {
            'c' => 0,
            'h' => 1,
            'o' => 2,
            'k' => 3,
            'u' => 4,
            'd' => 5,
            'a' => 6,
            'i' => 7,
            _ => continue,
        };
        if n == 0 {
            v[n] += 1;
        } else {
            v[n] = (v[n] + v[n - 1]) % MOD;
        }
    }
    println!("{}", v[7]);
}
