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
            aa: [Usize1; n * 2],
        };
        // let mut hs = HashSet::<_>::from_iter(0..n);
        let mut a2ii = vec![Vec::with_capacity(2); n];
        for (i, a) in aa.iter().copied().enumerate() {
            a2ii[a].push(i);
        }
        let mut rs = 0;
        for (a, ii) in a2ii.iter().enumerate() {
            if ii[0].abs_diff(ii[1]) == 1 {
                continue;
            }
            let mut bb = HashSet::new();
            for i in ii.iter().copied() {
                for j in [i.wrapping_sub(1), i + 1] {
                    if let Some(&b) = aa.get(j) {
                        if a < b {
                            bb.insert(b);
                        }
                    }
                }
            }
            for b in bb {
                let mut jj = a2ii[b].clone();
                if jj[0].abs_diff(jj[1]) == 1 {
                    continue;
                }
                for _ in 0..2 {
                    if izip!(ii, &jj).all(|(&i, &j)| i.abs_diff(j) == 1) {
                        rs += 1;
                        break;
                    }
                    jj.swap(0, 1);
                }
            }
        }
        println!("{rs}");
    }
}
