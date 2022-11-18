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
        a: usize,
        b: usize,
        hh: [usize; n],
    };
    let d = a - b;
    let mut ng = 0;
    let mut ok = 1e9 as usize + 10;
    let f = |mut k: usize| -> bool {
        let sub = k.saturating_mul(b);
        for &h in &hh {
            let x = (h.saturating_sub(sub)).div_ceil(d);
            if k < x {
                return false;
            }
            k -= x;
        }
        true
    };
    while ng + 1 < ok {
        let m = (ok + ng) / 2;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{ok}");
}
