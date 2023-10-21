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
        aa: [usize; n],
    };
    let mut xsum = vec![0];
    let mut asum = vec![0];
    for (i, &a) in aa.iter().enumerate() {
        xsum.push(xsum[i] ^ a);
        asum.push(asum[i] + a);
    }
    // eprintln!("{}", xsum.iter().join(" "));
    // eprintln!("{}", asum.iter().join(" "));
    let mut rs = 0usize;
    for l in 0..n {
        let mut ok = l;
        let mut ng = n;
        while ok + 1 != ng {
            let m = (ok + ng) / 2;

            let x = xsum[m + 1] ^ xsum[l];
            let a = asum[m + 1] - asum[l];
            if x == a {
                ok = m;
            } else {
                ng = m;
            }
        }
        // eprintln!("{l} {ok} {}", ok - l + 1);
        rs += ok - l + 1;
    }
    println!("{rs}");
}
