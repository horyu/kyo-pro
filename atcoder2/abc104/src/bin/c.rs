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
        d: usize,
        g: usize,
        ppcc: [(usize, usize); d],
    };
    let mut rs = std::usize::MAX;
    for ttff in (0..d).map(|_| [false, true]).multi_cartesian_product() {
        let mut tmp = 0;
        let mut score = 0;
        for (i, tf) in ttff.iter().copied().enumerate() {
            if tf {
                let (p, c) = ppcc[i];
                tmp += p;
                score += p * 100 * (i + 1) + c;
            }
        }
        if g <= score {
            rs = rs.min(tmp);
            continue;
        }
        for (i, tf) in ttff.iter().copied().enumerate().rev() {
            if !tf {
                let (p, c) = ppcc[i];
                let diff = g - score;
                let pp = diff.div_ceil(100 * (i + 1));
                if pp < p {
                    tmp += pp;
                    score += pp * 100 * (i + 1);
                }
                break;
            }
        }
        if g <= score {
            rs = rs.min(tmp);
        }
    }
    println!("{rs}");
}
