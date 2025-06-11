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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    // A | C
    // A | D
    // B | D

    // 方針
    // A | D を事前に配置する
    // Dの前後 D+1 個のスポットにCを配置する : ACD, ACDC, ADC
    // Aの前後+Dより前にあるCの前後にBを配置する : BABCBDC
    let comb = Combination::new(a + b + c + d, ModInt998244353::modulus() as usize);
    let mut rs = ModInt998244353::default();
    // Cを i個 Dの前に配置
    for i in 0..=c {
        // Dの間と後ろ d 箇所に c-i 個配置
        let mut tmp = comb.get(c - i + d - 1, c - i);
        // Aとi個のCの前後にBを配置
        tmp *= comb.get(a + i + b, b);
        rs += tmp;
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
