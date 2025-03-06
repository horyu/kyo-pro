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
        mut aa: [usize; n],
    };
    if aa[1] <= aa[0] {
        aa.reverse();
    }
    // a1/a0 = a[i+1]/a[i]
    // a1 * a[i] = a0 * a[i+1]
    let a0 = aa[0];
    let a1 = aa[1];
    let tf = aa
        .into_iter()
        .tuple_windows()
        .map(|(x, y)| a1 * x == a0 * y)
        .all_equal();
    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
