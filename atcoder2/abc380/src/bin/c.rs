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
        k: usize,
        s: Chars,
    };
    let mut ccww = s
        .into_iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, g)| (c, g.count()))
        .collect_vec();
    let from = 2 * (k - 1) + usize::from(ccww[0].0 == '0');
    ccww.swap(from - 1, from);
    let mut rs = String::new();
    for (c, w) in ccww {
        rs.push_str(&c.to_string().repeat(w));
    }
    println!("{rs}");
}
