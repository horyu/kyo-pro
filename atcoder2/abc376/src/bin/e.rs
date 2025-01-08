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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            aa: [usize; n],
            bb: [usize; n],
        }
        let mut rs = !0;
        let mut b_sum = 0;
        let mut bh = BinaryHeap::new();
        for (i, (a, b)) in izip!(aa, bb).sorted_unstable().enumerate() {
            b_sum += b;
            bh.push((b, a, i));
            if k < bh.len() {
                if let Some((bb, _, _)) = bh.pop() {
                    b_sum -= bb;
                }
            }
            if bh.len() == k {
                rs = rs.min(a * b_sum);
            }
        }
        println!("{rs}");
    }
}
