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
        q: usize,
    };
    // https://atcoder.jp/contests/abc428/editorial/14238
    let mut aa = VecDeque::from([0isize]);
    let mut bb = VecDeque::from([0isize]);
    for _ in 0..q {
        input! {t: usize};
        if t == 1 {
            input! {c: char};
            aa.push_front(aa[0] + if c == '(' { 1 } else { -1 });
            bb.push_front(aa[0].min(bb[0]));
        } else {
            aa.pop_front();
            bb.pop_front();
        }
        let rs = ["No", "Yes"][usize::from(aa[0] == 0 && bb[0] == 0)];
        println!("{rs}");
    }
}
