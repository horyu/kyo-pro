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
        l: usize,
    };
    const MOD: usize = 1e9 as usize + 7;
    let mut vv = vec![0; n + 1];
    vv[0] = 1;
    for i in 0..n {
        vv[i + 1] = (vv[i + 1] + vv[i]) % MOD;
        if i + l <= n {
            vv[i + l] = (vv[i + l] + vv[i]) % MOD;
        }
    }
    let rs = vv[n];
    println!("{rs}");
}
