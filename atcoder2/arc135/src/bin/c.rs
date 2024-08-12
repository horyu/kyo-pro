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
        mut aa: [isize; n],
    };
    // https://atcoder.jp/contests/arc135/editorial/3356

    aa.push(0);
    let mut ss = vec![0; n + 1];

    for k in 0..30 {
        let ones = aa.iter().copied().map(|a| (a >> k) & 1).collect_vec();
        let c1 = ones.iter().sum::<isize>();
        let c0 = n as isize - c1;

        let add = [c1 << k, (c1 - (c1 - c0)) << k];
        for i in 0..=n {
            ss[i] += add[ones[i] as usize];
        }
    }
    let rs = ss.iter().max().copied().unwrap();
    println!("{rs}");
}
