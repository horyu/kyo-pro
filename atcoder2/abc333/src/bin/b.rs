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
        s: Bytes,
        t: Bytes,
    };
    let x = (s[0].abs_diff(s[1]))
        .min(s[0].abs_diff(s[1] + 5))
        .min(s[0].abs_diff(s[1] - 5));
    let y = (t[0].abs_diff(t[1]))
        .min(t[0].abs_diff(t[1] + 5))
        .min(t[0].abs_diff(t[1] - 5));
    let rs = ["No", "Yes"][(x == y) as usize];
    println!("{rs}");
}
