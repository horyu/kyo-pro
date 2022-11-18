#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
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
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![false; n]; n];
    for (a, b) in aabb {
        g[a][b] = true;
        g[b][a] = true;
    }
    let rs = (1..n)
        .permutations(n - 1)
        .filter(|ii| g[0][ii[0]] && ii.iter().tuple_windows().all(|(&i, &j)| g[i][j]))
        .count();
    println!("{rs}");
}
