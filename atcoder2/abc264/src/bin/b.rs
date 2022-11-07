#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        r: Usize1,
        c: Usize1,
    };
    const T: bool = true;
    const F: bool = false;
    let tf = [
        [T, T, T, T, T, T, T, T, T, T, T, T, T, T, T],
        [T, F, F, F, F, F, F, F, F, F, F, F, F, F, T],
        [T, F, T, T, T, T, T, T, T, T, T, T, T, F, T],
        [T, F, T, F, F, F, F, F, F, F, F, F, T, F, T],
        [T, F, T, F, T, T, T, T, T, T, T, F, T, F, T],
        [T, F, T, F, T, F, F, F, F, F, T, F, T, F, T],
        [T, F, T, F, T, F, T, T, T, F, T, F, T, F, T],
        [T, F, T, F, T, F, T, F, T, F, T, F, T, F, T],
    ][r.min(14 - r)][c];
    let rs = if tf { "black" } else { "white" };
    println!("{rs}");
}
