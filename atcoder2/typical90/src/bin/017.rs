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
        m: usize,
        mut llrr: [(Usize1, Usize1); m],
    };
    llrr.sort_unstable_by_key(|&(l, r)| (l, std::cmp::Reverse(r)));
    let mut ft = ac_library::FenwickTree::new(n, 0usize);
    let mut rs = 0usize;
    for (l, r) in llrr {
        rs += ft.sum((l + 1)..r);
        ft.add(r, 1);
    }
    println!("{rs}");
}
