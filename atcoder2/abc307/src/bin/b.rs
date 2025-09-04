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
        ss: [Chars; n],
    };
    for ij in (0..n).permutations(2) {
        let i = ij[0];
        let j = ij[1];
        let vv = chain!(ss[i].iter().copied(), ss[j].iter().copied()).collect_vec();
        let ww = vv.iter().copied().rev().collect_vec();
        if vv == ww {
            print!("Yes");
            return;
        }
    }
    print!("No");
}
