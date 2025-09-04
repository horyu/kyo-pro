#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        m: usize,
        mut bb: [isize; n],
        mut ww: [isize; m],
    };
    bb.sort_unstable();
    ww.sort_unstable();
    let mut tmp = 0;
    let mut b_cnt = 0;
    while let Some(b) = bb.pop_if(|b| 0 <= *b) {
        tmp += b;
        b_cnt += 1;
    }
    let mut rs = tmp;
    let mut w_cnt = 0;
    while let Some(w) = ww.pop_if(|_| w_cnt < b_cnt) {
        w_cnt += 1;
        tmp += w;
        rs = rs.max(tmp);
    }
    while let (Some(b), Some(w)) = (bb.pop(), ww.pop()) {
        tmp += b;
        tmp += w;
        rs = rs.max(tmp);
    }
    println!("{rs}");
}
