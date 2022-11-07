#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        ppp: [[usize; w]; h],
    };
    let mut rs = 0;
    for hsize in 1..=h {
        for ii in (0..h).combinations(hsize) {
            let mut hm = HashMap::new();
            for j in 0..w {
                if ii.iter().map(|&i| ppp[i][j]).all_equal() {
                    *hm.entry(ppp[ii[0]][j]).or_insert(0) += 1usize;
                }
            }
            if let Some(m) = hm.into_values().max() {
                rs = rs.max(m * hsize);
            }
        }
    }
    println!("{rs}");
}
