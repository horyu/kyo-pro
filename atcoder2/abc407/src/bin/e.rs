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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            aa: [usize; 2 * n],
        };
        // https://atcoder.jp/contests/abc407/editorial/13106
        let mut rs = 0;
        let mut bh = BinaryHeap::new();
        for i in 0..n {
            if i == 0 {
                bh.push(aa[i]);
            } else {
                bh.push(aa[i * 2 - 1]);
                bh.push(aa[i * 2]);
            }
            rs += bh.pop().unwrap();
        }
        println!("{rs}");
    }
}
