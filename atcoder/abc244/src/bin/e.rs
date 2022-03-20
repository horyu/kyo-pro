#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 998244353;
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: usize,
        t: usize,
        x: usize,
        uuvv: [(usize, usize); m]
    };
    let mut vvv = vec![vec![]; n + 1];
    for (u, v) in uuvv {
        vvv[u].push(v);
        vvv[v].push(u);
    }
    let mut dd = vec![(0usize, 0usize); n + 1];
    dd[s].0 = 1;
    for _ in 0..k {
        let mut new_dd = vec![(0usize, 0usize); n + 1];
        for (i, (even, odd)) in dd.iter().enumerate() {
            for &v in &vvv[i] {
                if v == x {
                    new_dd[v] = ((new_dd[v].0 + odd) % MOD, (new_dd[v].1 + even) % MOD);
                } else {
                    new_dd[v] = ((new_dd[v].0 + even) % MOD, (new_dd[v].1 + odd) % MOD);
                }
            }
        }
        dd = new_dd;
    }
    println!("{}", dd[t].0);
}
