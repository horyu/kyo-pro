#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
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
    let mut ft = ac_library::FenwickTree::new(len, 0usize);

    let div = mod_pow(2, MOD - 2, MOD);
    let mut rs = 0;
    for (i, b) in bb.iter().copied().enumerate() {
        rs += ft.sum(..=b) * mod_pow(2, i, MOD);
        rs %= MOD;
        ft.add(b, mod_pow(div, i + 1, MOD));
    }
    println!("{rs}");
}

const MOD: usize = 998244353;

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
