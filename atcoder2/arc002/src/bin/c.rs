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
        cc: Chars,
    };
    let cc = cc
        .into_iter()
        .map(|c| match c {
            'A' => 0,
            'B' => 1,
            'X' => 2,
            _ => 3,
        })
        .collect_vec();
    let mut rs = n;
    for vv in (0..4).map(|_| [0, 1, 2, 3]).multi_cartesian_product() {
        let mut cnt = 0;
        let mut i = 0;
        while i < n - 1 {
            if cc[i..(i + 2)] == vv[0..=1] || cc[i..(i + 2)] == vv[2..=3] {
                cnt += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        rs = rs.min(n - cnt);
    }

    println!("{rs}");
}
