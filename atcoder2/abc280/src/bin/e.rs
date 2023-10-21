#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::{Mod998244353, ModInt998244353};
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
        p: usize,
    };
    // 2:p 1:q
    // n  回 [q^(n-1)    ]*[p,q]
    // n-1回 [q^(n-3),p^1]*[p,q], q^(n-2)*p
    // n-2回 [q^(n-5),p^2]*[p,q], [q^(n-4), p^1]*p
    let comb = Combination::new(n + 1, 998244353);
    let divp = ModInt998244353::new(p) / 100;
    let divq = ModInt998244353::new(100 - p) / 100;
    let mut rs = ModInt998244353::default();
    for c in n.div_ceil(2)..=n {
        let cc = c - 1;
        let mut tmp = ModInt998244353::default();
        // from: n - 2
        if 2 <= n && (cc..=(2 * cc)).contains(&(n - 2)) {
            // 2a + b = n - 2;
            //  a + b = cc
            let a = n - 2 - cc;
            let b = 2 * cc - (n - 2);
            tmp += divp.pow(a as u64) * divq.pow(b as u64) * comb.get(cc, a) * divp;
        }
        // from: n - 1
        if (cc..=(2 * cc)).contains(&(n - 1)) {
            // 2a + b = n - 1;
            //  a + b = cc;
            let a = n - 1 - cc;
            let b = 2 * cc - (n - 1);
            tmp += divp.pow(a as u64) * divq.pow(b as u64) * comb.get(cc, a);
        }
        rs += tmp * c;
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
