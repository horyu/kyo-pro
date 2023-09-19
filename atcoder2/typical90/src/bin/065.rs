#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
        x: usize,
        y: usize,
        z: usize,
    };
    // cr+cg <= x
    // cg+cb <= y
    // cb+cr <= z
    // cr-cb <= x-y
    // cg-cr <= y-z
    // cb-cg <= z-x
    // cr <= (x-y+z)/2
    // cg <= (y-z+x)/2
    // cb <= (z-x+y)/2
    // cr+cb+cg = k
    // k-x <= cb
    // k-y <= cg
    // k-z <= cr
    let min_r = k - y;
    let min_g = k - z;
    let min_b = k - x;

    let comb = Combination::new(r.max(g).max(b), ModInt998244353::modulus() as usize);
    let ar = (0..=r).map(|i| comb.get(r, i)).collect_vec();
    let ag = (0..=g).map(|i| comb.get(g, i)).collect_vec();
    let ab = (0..=b).map(|i| comb.get(b, i)).collect_vec();

    let mut p1 = vec![ModInt998244353::default(); r + 1];
    for i in min_r..=r {
        p1[i] += ar[i];
    }
    let mut p2 = vec![ModInt998244353::default(); g + 1];
    for i in min_g..=g {
        p2[i] += ag[i];
    }
    let p3 = ac_library::convolution(&p1, &p2);

    let mut rs = ModInt998244353::default();
    for i in min_b..=b {
        let ret1 = p3.get(k - i).copied().unwrap_or_default();
        let ret2 = ab[i];
        rs += ret1 * ret2;
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
