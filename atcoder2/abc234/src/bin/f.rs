#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s: Bytes,
    };
    let n = s.len();
    let mut vv = [0; 26];
    for b in s {
        vv[(b - b'a') as usize] += 1;
    }

    const MOD: usize = 998244353;
    let comb = Combination::new(n, MOD);
    let mut dp = vec![vec![0; n + 1]; 27];
    dp[0][0] = 1;
    for i in 0..26 {
        for j in 0..=n {
            for k in 0..=(j.min(vv[i])) {
                dp[i + 1][j] += dp[i][j - k] * comb.get(j, k);
                dp[i + 1][j] %= MOD;
            }
        }
    }
    let mut rs = ModInt998244353::default();
    for i in 1..=n {
        rs += dp[26][i];
    }
    println!("{rs}");
}

// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/combination.rs
pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Self {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x >= y);
        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }

    pub fn h(&self, n: usize, r: usize) -> usize {
        self.get(n + r - 1, r)
    }
}
