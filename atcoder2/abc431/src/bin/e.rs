#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            h: usize,
            w: usize,
            ss: [Chars; h],
        };
        // マスの辺を以下のように番号付けする
        // ┏0┓
        // 1 2
        // ┗3┛
        // A: 　  0-3, 1-2
        // B: ／  0-1, 2-3
        // C: ＼  0-2, 1-3
        // あとは Union-Find でつなげていき、ss[0][0][1] と ss[h-1][w-1][2] の間にあるグループ数を数える？
    }
    // println!("{rs}");
}
