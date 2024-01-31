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
        h: usize,
        w: usize,
        k: usize,
        aa: [usize; h],
        bb: [usize; w],
    };
    // https://atcoder.jp/contests/arc133/editorial/3282
    let cc = aa
        .iter()
        .copied()
        .map(|a| ((k - 1) * w - a) % k)
        .collect_vec();
    let dd = bb
        .iter()
        .copied()
        .map(|b| ((k - 1) * h - b) % k)
        .collect_vec();
    let c_sum = cc.iter().sum::<usize>();
    let d_sum = dd.iter().sum::<usize>();
    if c_sum % k != d_sum % k {
        println!("-1");
    } else {
        let rs = h * w * (k - 1) - c_sum.max(d_sum);
        println!("{rs}");
    }
}
