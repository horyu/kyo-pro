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
        h: usize,
        xx: [usize; n],
        ppff: [(usize, usize); n - 1],
    };
    let xn = xx[n - 1];
    if xn * 2 <= h {
        println!("0");
        return;
    }
    let mut rs = !0;
    // 往路
    for (x, (p, f)) in izip!(xx.iter().copied(), ppff.iter().copied()) {
        if h < x {
            continue;
        }
        let hh = (h - x + f).min(h);
        eprintln!("-> {x} {p} {f} {hh} {}", 2 * xn - x);
        if 2 * xn - x <= hh {
            rs = rs.min(p);
        }
    }
    // 復路
    for (x, (p, f)) in izip!(xx.iter().copied(), ppff.iter().copied()) {
        if h < 2 * xn - x {
            continue;
        }
        let hh = (h - (2 * xn - x) + f).min(h);
        if x <= hh {
            rs = rs.min(p);
        }
    }
    if rs == !0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
