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
    };
    let mut btm = BTreeMap::new();
    btm.insert(1, ModInt998244353::new(1));
    let r = ModInt998244353::new(1) / 5;
    while let Some((k, v)) = btm.pop_first() {
        if k == n {
            println!("{v}");
            return;
        }
        for i in 2..=6 {
            let kk = k * i;
            let vv = v * r;
            if kk <= n {
                *btm.entry(kk).or_default() += vv;
            }
        }
    }
    println!("0");
}
