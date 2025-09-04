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
        xxyy: [(isize, isize); n],
    };
    let mut xy2i: HashMap<(isize, isize), usize> = HashMap::new();
    for (i, &(x, y)) in xxyy.iter().enumerate() {
        xy2i.insert((x, y), i);
    }
    let mut uf = UnionFind::new(n);
    for (i, (x, y)) in xxyy.into_iter().enumerate() {
        for (dx, dy) in [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)] {
            if let Some(&j) = xy2i.get(&(x + dx, y + dy)) {
                uf.union(i, j);
            }
        }
    }
    let rs = uf
        .into_labeling()
        .into_iter()
        .enumerate()
        .filter(|&(i, l)| i == l)
        .count();
    println!("{rs}");
}
