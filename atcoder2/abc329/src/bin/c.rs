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
        s: Chars,
    };
    let mut counter = counter::Counter::<_>::new();
    for (k, g) in s.iter().copied().group_by(|&c| c).into_iter() {
        let count = g.count();
        counter[&k] = count.max(counter[&k]);
    }
    let mut rs = 0;
    for v in counter.values() {
        rs += v;
    }

    println!("{rs}");
}
