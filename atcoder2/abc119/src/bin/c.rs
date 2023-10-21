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
        // a: usize,
        // b: usize,
        // c: usize,
        abc: [isize; 3],
        ll: [isize; n],
    };
    let mut rs = std::usize::MAX;
    for vv in (0..n).map(|_| 0..4).multi_cartesian_product() {
        let mut cnts = [0; 4];
        for &v in &vv {
            cnts[v] += 1;
        }
        if cnts[..3].iter().all(|&v| 0 < v) {
            let mut ww = [0; 4];
            for (i, v) in vv.iter().copied().enumerate() {
                ww[v] += ll[i];
            }
            let tmp = (0..3).fold(0, |acc, i| {
                acc + (cnts[i] - 1) * 10 + abc[i].abs_diff(ww[i])
            });
            rs = rs.min(tmp);
        }
    }
    println!("{rs}");
}
