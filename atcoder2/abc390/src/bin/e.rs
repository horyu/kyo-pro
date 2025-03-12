#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
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
    let mut aaa = vec![vec![0; x + 1]; 3];
    for (v, a, c) in vvaacc {
        let aa = &mut aaa[v];
        for i in ((c + 1)..=x).rev() {
            if 0 < aa[i - c] {
                aa[i] = aa[i].max(aa[i - c] + a);
            }
        }
        aa[c] = aa[c].max(a);
    }
    let bbttmm = aaa
        .into_iter()
        .map(|aa| {
            let mut btm = BTreeMap::new();
            let mut max = 0;
            for (c, a) in aa.into_iter().enumerate() {
                if max < a {
                    btm.insert(a, c);
                }
                max = max.max(a);
            }
            btm
        })
        .collect_vec();
    // for btm in &bbttmm {
    //     eprintln!("{btm:?}");
    // }
    let mut ok = 0;
    let mut ng = bbttmm
        .iter()
        .flat_map(|btm| btm.keys().max())
        .min()
        .unwrap()
        + 1;
    while 1 < ng - ok {
        let mid = ok.midpoint(ng);
        let mut sum = 0;
        for btm in bbttmm.iter() {
            if let Some((_a, c)) = btm.range(mid..).next() {
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
