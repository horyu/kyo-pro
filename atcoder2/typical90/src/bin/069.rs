#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        k: usize,
    };
    const MOD: usize = 1e9 as usize + 7;
    let mut rs = k;
    if 2 <= n {
        rs *= k.saturating_sub(1);
        rs %= MOD;
    }
    if 3 <= n {
        rs *= mod_pow(k.saturating_sub(2), n - 2, MOD);
        rs %= MOD;
    }
    println!("{rs}");
}

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
