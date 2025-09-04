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
        m: usize,
        cc: [usize; n],
    };
    let mut i2jj = vec![vec![]; n];
    for j in 0..m {
        input! {k: usize, ii: [Usize1; k]};
        for i in ii {
            i2jj[i].push(j);
        }
    }
    let mut rs = !0;
    for bits1 in 0..(1usize << n) {
        let mut xx = vec![0; m];
        let mut c1 = 0;
        for i in 0..n {
            if bits1 & (1 << i) != 0 {
                c1 += cc[i];
                for j in i2jj[i].iter().copied() {
                    xx[j] += 1;
                }
            }
        }
        if xx.iter().all(|&x| 2 <= x) {
            rs = rs.min(c1);
            continue;
        }
        for bits2 in bits1..(1usize << n) {
            let mut c2 = c1;
            let mut yy = xx.clone();
            for i in 0..n {
                if bits2 & (1 << i) != 0 {
                    c2 += cc[i];
                    for j in i2jj[i].iter().copied() {
                        yy[j] += 1;
                    }
                }
            }
            if yy.iter().all(|&y| 2 <= y) {
                rs = rs.min(c2);
            }
        }
    }
    println!("{rs}");
}
