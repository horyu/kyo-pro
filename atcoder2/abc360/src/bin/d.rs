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
        t: isize,
        s: Chars,
        xx: [isize; n],
    };
    let mut ll = vec![];
    let mut rr = vec![];
    for (c, x) in izip!(s, xx) {
        if c == '0' {
            ll.push(x);
        } else {
            rr.push(x);
        }
    }
    rr.sort_unstable();
    let mut rs = 0;
    for l in ll {
        // rr の中に (l-2t)..=l に含まれる数を求める
        rs += rr.partition_point(|&r| r <= l) - rr.partition_point(|&r| r < l - 2 * t);
    }
    println!("{rs}");
}
