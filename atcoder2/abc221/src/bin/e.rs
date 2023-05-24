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
        n: usize,
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc221/editorial/2718
    let a2i: HashMap<usize, usize> = aa
        .iter()
        .copied()
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect();
    let bb = aa
        .iter()
        .copied()
        .map(|a| a2i.get(&a).copied().unwrap())
        .collect_vec();
    // eprintln!("{bb:?}");

    let len = a2i.len();
    let mut bit = Bit::new(len);

    let div = mod_pow(2, MOD - 2, MOD);
    let mut rs = 0;
    for (i, b) in bb.iter().copied().enumerate() {
        rs += bit.sum(b) * mod_pow(2, i, MOD);
        rs %= MOD;
        bit.add(b, mod_pow(div, i + 1, MOD));
    }
    println!("{rs}");
}

const MOD: usize = 998244353;

struct Bit {
    n: usize,
    bit: Vec<usize>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }
    fn addition(&self, x: usize, y: usize) -> usize {
        (x + y) % MOD
    }
    fn add(&mut self, mut x: usize, a: usize) {
        x += 1;
        while x <= self.n {
            self.bit[x] = self.addition(self.bit[x], a);
            x += x & x.wrapping_neg();
        }
    }
    fn sum(&self, mut x: usize) -> usize {
        x += 1;
        let mut ret = 0;
        while 0 < x {
            ret = self.addition(ret, self.bit[x]);
            x -= x & x.wrapping_neg();
        }
        ret
    }
}

// https://blog.spiralray.net/cp/modulo#i-8
fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
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
