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

fn main() {
    input! {
        n: usize,
        x: usize,
        uudd: [(Usize1, Usize1); n],
    };
    // https://atcoder.jp/contests/abc395/editorial/12344
    let mut r = uudd.iter().map(|(u, d)| u + d).min().unwrap() + 1;
    let mut l = x.min(r - 1);
    while l + 1 < r {
        let m = l.midpoint(r);
        let mut tf = true;
        let mut ll = 0usize;
        let mut rr = m;
        for (u, d) in uudd.iter().copied() {
            // [L - X, R + X] & [m - D, U]
            ll = (ll.saturating_sub(x)).max(m.saturating_sub(d));
            rr = (rr + x).min(u);
            if rr < ll {
                tf = false;
                break;
            }
        }
        if tf {
            l = m;
        } else {
            r = m;
        }
    }
    let rs = uudd.iter().fold(0, |acc, (u, d)| acc + u + d) - l * n;
    println!("{rs}");
}
