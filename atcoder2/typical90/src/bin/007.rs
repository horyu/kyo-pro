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
        aa: [isize; n],
        q: usize,
        bb: [isize; q],
    };
    let bts = BTreeSet::from_iter(aa);
    for b in bb {
        let r = bts.range(b..).next().copied().unwrap_or(isize::MAX >> 2);
        let l = bts
            .range(..b)
            .next_back()
            .copied()
            .unwrap_or(isize::MAX >> 2);
        let rs = (r - b).abs().min((l - b).abs());
        println!("{rs}");
    }
}
