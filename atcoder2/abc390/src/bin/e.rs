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
        x: usize,
        vvaacc: [(Usize1, usize, usize); n],
    };
    let mut ss = [0; 3];
    let mut ddd = vec![vec![0; x + 1]; 3];
    for (v, a, c) in vvaacc.into_iter().sorted_unstable_by_key(|vac| vac.2) {
        ss[v] += a;
        for i in ((c + 1)..=x).rev() {
            if 0 < ddd[v][i - c] {
                ddd[v][i] = ddd[v][i].max(ddd[v][i - c] + a);
            }
        }
        ddd[v][c] = ddd[v][c].max(a);
    }
    let mut bbttmm = vec![BTreeMap::new(); 3];
    for (v, dd) in ddd.into_iter().enumerate() {
        for (c, d) in dd.into_iter().enumerate() {
            if 0 < d {
                bbttmm[v].insert(d, c);
            }
        }
    }
    // for btm in &bbttmm {
    //     eprintln!("{btm:?}");
    // }
    let mut ok = 0;
    let mut ng = ss.into_iter().min().unwrap() + 1;
    while 1 < ng - ok {
        // 目標ビタミン
        let mid = ok.midpoint(ng);
        let mut sum = 0;
        for btm in bbttmm.iter() {
            if let Some((_d, c)) = btm.range(mid..).next() {
                sum += c;
            } else {
                sum = x + 1;
                break;
            }
        }
        if sum <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let rs = ok;
    println!("{rs}");
}
