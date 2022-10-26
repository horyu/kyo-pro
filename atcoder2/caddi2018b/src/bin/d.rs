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
        n: usize,
        aa: [usize; n],
    };
    // この手番で勝つ
    // 1; 1,1,..,1
    // どうしても負け
    // 2; 4; 2,2

    // 22
    // 12 11
    // 02 x
    // 01
    // x

    // 13 -> 02 -> o
    // 23 -> 22 -> o

    let tf = aa.into_iter().all(|a| a.is_even());
    let rs = if tf { "second" } else { "first" };
    println!("{rs}");
}
