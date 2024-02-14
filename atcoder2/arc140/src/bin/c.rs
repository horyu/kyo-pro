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
    };
    // https://atcoder.jp/contests/arc140/submissions/32062669
    let mut rrss = vec![x];
    let vv = chain(1..x, (x + 1)..=n).collect_vec();
    let m = vv.len() / 2;
    if vv.len() % 2 == 1 {
        rrss.push(vv[m]);
    }
    for i in 0..m {
        let (p, q) = (vv[m - 1 - i], vv[m + vv.len() % 2 + i]);
        if n % 2 == 0 && x == (n / 2 + 1) {
            rrss.push(q);
            rrss.push(p);
        } else {
            rrss.push(p);
            rrss.push(q);
        }
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
