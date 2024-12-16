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
        xx: [isize; n],
        pp: [usize; n],
        q: usize,
        llrr: [(isize, isize); q],
    };
    let x2i = llrr
        .iter()
        .copied()
        .flat_map(|(l, r)| [l, r])
        .chain(xx.iter().copied())
        .sorted_unstable()
        .dedup()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    let size = x2i.len();
    let mut ft = ac_library::FenwickTree::new(size, 0usize);
    for (x, p) in izip!(xx, pp) {
        let i = x2i[&x];
        ft.add(i, p);
    }
    for (l, r) in llrr {
        let l = x2i[&l];
        let r = x2i[&r];
        let rs = ft.sum(l..=r);
        println!("{rs}");
    }
}
