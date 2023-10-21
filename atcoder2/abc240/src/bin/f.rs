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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            xxyy: [(isize, isize); n],
        };
        let mut v = 0;
        let mut pos = 0;
        let mut rs: isize = xxyy[0].0;
        for (x, y) in xxyy {
            let nv = v + x * y;
            // eprintln!("v: {v}->{nv}");
            if nv < 0 && 0 < v {
                let t = (v / x).abs();
                rs = rs.max(pos + v * t + x * t * (t + 1) / 2);
            }
            pos += v * y + x * y * (y + 1) / 2;
            // dbg!(pos);
            v = nv;
            rs = rs.max(pos);
        }
        println!("{rs}");
    }
}
