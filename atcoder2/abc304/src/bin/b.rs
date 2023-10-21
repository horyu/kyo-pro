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
    };

    let ilog10 = n.checked_ilog10().unwrap_or_default();
    let mut rs = n;
    if 3 <= ilog10 {
        rs = rs / 10usize.pow(ilog10 - 2) * 10usize.pow(ilog10 - 2);
    }
    println!("{rs}");
}
