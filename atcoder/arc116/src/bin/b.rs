#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

const MOD: usize = 998244353;
fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    // 2 3 4
    // 2 * (2 + 3 + 4 + 4)
    // 3 * (3 + 4)
    // 4 * (4)

    // 1234
    // 1,2,3,4
    // 12,13,14,23,24,34
    // 123,124,134,234,
    // 1234
    // MINから
    // 1*(1+2+3+3+4+4+4+4+4)
    // 2*(2+3+4+4)
    // 3*(3*4)
    // 4*(4)
    // MAXから
    // 1*(1)
    // 2*(1+2)
    // 3*(1+1+2+3)
    // 4*(1+1+1+1+2+2+3+4)

    aa.sort_unstable();
    let mut mul = 0;
    let mut rs = 0;
    for a in aa {
        rs = (rs + a * ((a + mul) % MOD) % MOD) % MOD;
        mul = (mul * 2 + a) % MOD;
    }
    println!("{rs}");
}
