#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        ttaa: [(usize, usize); n],
    };
    let mut x = 1usize;
    let mut y = 1usize;
    for (t, a) in ttaa {
        let mul = x.div_ceil(t).max(y.div_ceil(a));
        // dbg!(mul);
        let mut xx = t * mul;
        let mut yy = a * mul;
        for m in mul.. {
            if x <= xx && y <= yy {
                break;
            }
            xx = t * m;
            yy = a * m;
        }
        x = xx;
        y = yy;
        // eprintln!("{x} {y}");
    }
    let rs = x + y;
    println!("{rs}");
}
