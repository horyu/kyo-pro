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
        n: usize,
        wwhhbb: [(usize, usize, usize); n],
    };
    // これだと 2^n 通りでTLEする
    let mut hs = HashSet::new();
    hs.insert((0, 0, 0));
    for (w, h, b) in wwhhbb {
        let mut new_hs = HashSet::new();
        for (x, wh, wb) in hs {
            new_hs.insert((x + h, wh + w, wb));
            new_hs.insert((x + b, wh, wb + w));
        }
        hs = new_hs;
    }
    let rs = hs
        .into_iter()
        .filter_map(|(x, wh, wb)| (wh <= wb).then_some(x))
        .max()
        .unwrap_or_default();
    println!("{rs}");
}
