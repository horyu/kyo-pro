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
        // a: (isize, isize),
        // b: (isize, isize),
        // c: (isize, isize),
        abc: [(isize, isize); 3],
    };
    // 三平方の定理
    let mut pows = vec![];
    for (p, q) in abc.iter().copied().cycle().tuple_windows().take(3) {
        pows.push((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2));
    }
    pows.sort_unstable();
    let tf = pows[0] + pows[1] == pows[2];
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
