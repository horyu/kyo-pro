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
    };
    let ss = chain!([0], aa.into_iter().cycle().take(2 * n))
        .cumsum::<usize>()
        .collect_vec();
    let mut shift = 0;
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {c: usize};
            shift = (shift + c) % n;
        } else {
            input! {l: Usize1, r: Usize1};
            let rs = ss[r + 1 + shift] - ss[l + shift];
            println!("{rs}");
        }
    }
}
