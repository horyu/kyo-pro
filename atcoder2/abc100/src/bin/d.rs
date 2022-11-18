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
        n: usize,
        m: usize,
        mut xxyyzz: [[isize; 3]; n],
    };
    let mut rs = 0isize;
    for vv in [
        [1, 1, 1],
        [1, 1, -1],
        [1, -1, 1],
        [1, -1, -1],
        [-1, 1, 1],
        [-1, 1, -1],
        [-1, -1, 1],
        [-1, -1, -1],
    ] {
        xxyyzz.sort_unstable_by_key(|xyz| {
            let mut tmp = 0;
            for i in 0..3 {
                tmp -= vv[i] * xyz[i];
            }
            tmp
        });
        let mut ww = [0; 3];
        for xyz in &xxyyzz[..m] {
            for i in 0..3 {
                ww[i] += xyz[i];
            }
        }
        rs = rs.max(ww.into_iter().map(|w| w.abs()).sum::<isize>());
    }
    println!("{rs}");
}
