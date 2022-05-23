#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    const MOD: usize = 1e9 as usize + 7;
    let mut vv = [0; 7];
    for c in s {
        match c {
            'a' => {
                vv[0] += 1;
            },
            't' => {
                vv[1] = (vv[1] + vv[0]) % MOD;
            },
            'c' => {
                vv[2] = (vv[2] + vv[1]) % MOD;
            },
            'o' => {
                vv[3] = (vv[3] + vv[2]) % MOD;
            },
            'd' => {
                vv[4] = (vv[4] + vv[3]) % MOD;
            },
            'e' => {
                vv[5] = (vv[5] + vv[4]) % MOD;
            },
            'r' => {
                vv[6] = (vv[6] + vv[5]) % MOD;
            },
            _ => (),
        }
    }
    println!("{}", vv[6]);
}
