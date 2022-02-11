#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize
    };
    let mut vv = vec![0; 1e6 as usize + 2];
    vv[1] = 7 % k;
    for i in 2..=k {
        vv[i] = (vv[i - 1] * 10 + 7) % k;
    }
    #[allow(clippy::needless_range_loop)]
    for i in 1..=k {
        if vv[i] == 0 {
            println!("{i}");
            return;
        }
    }
    println!("-1");
}
