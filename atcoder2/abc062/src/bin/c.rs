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
        mut h: usize,
        mut w: usize,
    };
    let mut rs = std::usize::MAX;
    // T字に切る
    for _ in 0..2 {
        let jl = w / 2;
        let jr = w - jl;
        for i in 1..=h.div_ceil(2) {
            let ss = vec![w * i, (h - i) * jl, (h - i) * jr];
            rs = rs.min(ss.iter().max().unwrap() - ss.iter().min().unwrap());
        }
        std::mem::swap(&mut h, &mut w);
    }
    // =で切る
    for _ in 0..2 {
        if 3 <= h {
            let i0 = h / 3;
            let i2 = h.div_ceil(3);
            let ss = vec![w * i0, w * i2];
            rs = rs.min(ss.iter().max().unwrap() - ss.iter().min().unwrap());
        }
        std::mem::swap(&mut h, &mut w);
    }
    println!("{rs}");
}
