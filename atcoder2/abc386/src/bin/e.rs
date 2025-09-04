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
        k: usize,
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc386/editorial/11697
    let mut rs = 0;
    if k <= n - k {
        for vv in aa.into_iter().combinations(k) {
            rs = rs.max(vv.into_iter().fold(0, |acc, v| acc ^ v));
        }
    } else {
        let xor_xum = aa.iter().copied().fold(0, |acc, v| acc ^ v);
        for vv in aa.into_iter().combinations(n - k) {
            rs = rs.max(vv.into_iter().fold(xor_xum, |acc, v| acc ^ v));
        }
    }
    println!("{rs}");
}
