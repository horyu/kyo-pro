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
        n: u64,
        m: u64,
        k: u64,
    };
    if n.min(m) == 1 {
        let rs = ModInt998244353::new(k).pow(n.max(m));
        println!("{rs}");
        return;
    }
    // https://atcoder.jp/contests/arc113/editorial/709
    let mut rs = ModInt998244353::default();
    for x in 1..=k {
        rs += (ModInt998244353::new(x).pow(n) - ModInt998244353::new(x - 1).pow(n))
            * ModInt998244353::new(k - x + 1).pow(m);
    }
    println!("{rs}");
}
