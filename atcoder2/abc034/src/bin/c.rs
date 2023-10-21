#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        w: usize,
        h: usize,
    };
    // 横にw-1回 縦にh-1回移動の並び替え
    let mut rs = ModInt1000000007::new(1);
    let ww = w - 1;
    let hh = h - 1;
    for i in 1..=(hh + ww) {
        rs *= i;
    }
    for i in 1..=hh {
        rs /= i;
    }
    for i in 1..=ww {
        rs /= i;
    }
    println!("{rs}");
}
