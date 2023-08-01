#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    };
    let mut pp = vec![vec![]; n];
    for (i, a) in aa.iter().copied().enumerate() {
        pp[a].push(i);
    }
    let mut rs = 0usize;
    for i in 1..=n {
        rs += (n + 1 - i) * (i / 2);
    }
    for pp in pp {
        let mut l = 0;
        let mut r = pp.len().saturating_sub(1);
        while l < r {
            if pp[l] + 1 < n - pp[r] {
                rs -= (r - l) * (pp[l] + 1);
                l += 1;
            } else {
                rs -= (r - l) * (n - pp[r]);
                r -= 1;
            }
        }
    }
    println!("{rs}");
}
