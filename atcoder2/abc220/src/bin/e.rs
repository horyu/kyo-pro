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
        d: usize,
    };
    let pows = (0..=(3e6 as u64))
        .map(|k| ModInt998244353::new(2).pow(k))
        .collect_vec();
    let zero = ModInt998244353::default();
    let mut rs = ModInt998244353::default();
    for x in 0..n {
        let f1 = if x + d <= n - 1 { pows[d + 1] } else { zero };
        let f2 = if 2 * (n - 1 - x) < d || d == 1 {
            zero
        } else if x + d <= n - 1 {
            pows[d - 1] * (d - 1)
        } else {
            pows[d - 1] * (2 * n - 2 * x - d - 1)
        };
        rs += pows[x] * (f1 + f2);
    }
    println!("{rs}");
}
