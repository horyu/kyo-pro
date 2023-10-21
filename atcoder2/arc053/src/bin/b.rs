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
        s: Chars
    };
    let hm = s.into_iter().counts();
    let mut odd = 0;
    let mut even = 0;
    for v in hm.values() {
        odd += v % 2;
        even += v / 2;
    }
    let rs = if odd <= 1 {
        odd + even * 2
    } else {
        even / odd * 2 + 1
    };
    println!("{rs}");
}
