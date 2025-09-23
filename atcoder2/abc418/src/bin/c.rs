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
        q: usize,
        aa: [usize; n],
        bb: [usize; q],
    };
    let size = 1e6 as usize + 2;
    // cc[i] := i以下のaaの要素数
    let mut cc = vec![0usize; size];
    // dd[i] := i以下のaaの要素和
    let mut dd = vec![0usize; size];
    for a in aa.iter().copied() {
        cc[a] += 1;
        dd[a] += a;
    }
    for i in 1..size {
        cc[i] += cc[i - 1];
        dd[i] += dd[i - 1];
    }
    for b in bb {
        // b未満の要素和 + b以上の要素数 * (b - 1) + 1
        let rs = dd[b - 1] + (cc[size - 1] - cc[b - 1]) * (b - 1) + 1;
        if dd[size - 1] < rs {
            println!("-1");
            continue;
        }
        println!("{rs}");
    }
}
