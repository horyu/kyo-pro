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
        llrr: [(usize, usize); n],
    };
    let mut rs = 0.0;
    for ((li, ri), (lj, rj)) in llrr.iter().copied().tuple_combinations() {
        let mut cnt = 0;
        for i in li..=ri {
            for j in lj..=rj {
                if j < i {
                    cnt += 1;
                }
            }
        }
        rs += cnt as f64 / ((ri - li + 1) * (rj - lj + 1)) as f64;
    }

    println!("{rs}");
}
