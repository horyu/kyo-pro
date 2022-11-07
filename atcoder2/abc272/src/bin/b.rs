#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut hs: HashSet<(usize, usize)> = (0usize..n).tuple_combinations().collect();
    for _ in 0..m {
        input! {
            k: usize,
            xx: [Usize1; k],
        };
        for (x, y) in xx.into_iter().tuple_combinations() {
            hs.remove(&(x.min(y), x.max(y)));
        }
    }
    let rs = if hs.is_empty() { "Yes" } else { "No" };
    println!("{rs}");
}
