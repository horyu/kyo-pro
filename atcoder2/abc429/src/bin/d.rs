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
        m: usize,
        c: usize,
        mut aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc429/editorial/14279
    aa.sort_unstable();
    let (pp, bb): (Vec<usize>, Vec<usize>) = aa.iter().copied().dedup_with_count().unzip();
    let k = bb.len();
    let mut r = 0;
    let mut cur = 0;
    let mut rs = 0usize;
    for i in 0..k {
        while cur < c {
            cur += pp[r];
            r += 1;
            if k <= r {
                r -= k;
            }
        }
        if i == 0 {
            // 円の先頭と末尾部分
            rs += (m + bb[0] - bb[k - 1]) * cur;
        } else {
            rs += (bb[i] - bb[i - 1]) * cur;
        }
        cur -= pp[i];
    }
    println!("{rs}");
}
