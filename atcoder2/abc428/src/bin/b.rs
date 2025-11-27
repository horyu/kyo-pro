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
        k: usize,
        s: Chars,
    };
    let vv = s
        .windows(k)
        .sorted_unstable()
        .dedup_with_count()
        .max_set_by_key(|(cnt, _)| *cnt);
    println!("{}", vv[0].0);
    println!(
        "{}",
        vv.iter()
            .map(|(_, s)| s.iter().join(""))
            .sorted_unstable()
            .join(" ")
    );
}
