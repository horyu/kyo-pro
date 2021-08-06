#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    const MOD: isize = 1_000_000_007;
    let mut dpp = vec![0; n];
    let mut dpn = vec![0; n];
    dpp[0] = 1;
    dpn[0] = 0;
    for i in 1..n {
        dpp[i] = (dpp[i - 1] + dpn[i - 1]) % MOD;
        dpn[i] = dpp[i - 1];
    }
    let (p, n) = (1..n).fold((aa[0], 0), |(p, n), i| {
        (
            (p + dpp[i - 1] * aa[i] + n + dpn[i - 1] * aa[i]) % MOD,
            p - dpp[i - 1] * aa[i],
        )
    });
    println!("{}", (p + n).rem_euclid(MOD));
}
