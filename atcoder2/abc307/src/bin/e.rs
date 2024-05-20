#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        n: usize,
        m: usize,
    };
    // https://atcoder.jp/contests/abc307/editorial/6643
    // 包除原理
    const MOD: usize = 998244353;
    let comb = Combination::new(n, MOD);
    let mut rs = ModInt998244353::default();
    for k in 0..=n {
        let r = mod_pow(m, n.saturating_sub(k + 1) + 1, MOD) * comb.get(n, k);
        if k.is_odd() {
            rs -= r;
        } else {
            rs += r;
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
    };
    // dp = i人目が0人目と [同じ組み合わせ数, 異なる組み合わせ数]
    let mut dp = [ModInt998244353::new(0); 2];
    dp[0] += m;
    for i in 1..n {
        dp = [dp[1], dp[0] * (m - 1) + dp[1] * (m - 2)];
    }
    let rs = dp[1];
    println!("{rs}");
}

// https://blog.spiralray.net/cp/modulo#i-8
fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
    x %= m;
    let mut ans = 1;
    while n != 0 {
        if n.is_odd() {
            ans = ans * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ans
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
