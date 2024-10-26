#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

/*
f(x) =||x-a|-b|
*/

fn main() {
    input! {
        q: usize,
        a: isize,
        b: isize,
        ttaabb: [(usize, isize, isize); q],
    };
    // https://atcoder.jp/contests/arc155/editorial/5594
    let mut bts = BTreeSet::from_iter([a + b, a - b]);
    for (t, a, b) in ttaabb {
        if t == 1 {
            bts.insert(a + b);
            bts.insert(a - b);
            continue;
        }

        let mut rs = 1 << 60;
        if bts.range(a..=b).next().is_some() {
            rs = 0;
        } else {
            if let Some(&x) = bts.range(..a).next_back() {
                rs = rs.min((a - x).abs());
            }
            if let Some(&x) = bts.range(b..).next() {
                rs = rs.min((b - x).abs());
            }
        }
        println!("{rs}");
    }
}
