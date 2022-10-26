#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        mut aa: [usize; 5],
    };
    aa.sort_unstable();
    let mut tf = aa[0] == aa[2] && aa[2] != aa[3] && aa[3] == aa[4];
    if !tf {
        aa.reverse();
        tf = aa[0] == aa[2] && aa[2] != aa[3] && aa[3] == aa[4]
    }
    let rs = if tf { "Yes" } else { "No" };
    println!("{rs}");
}
