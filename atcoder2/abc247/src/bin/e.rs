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
        x: usize,
        y: usize,
        aa: [usize; n],
    };
    let mut rs = 0;
    let gg = aa
        .into_iter()
        .group_by(|&a| (y..=x).contains(&a))
        .into_iter()
        .filter_map(|g| if g.0 { Some(g.1.collect_vec()) } else { None })
        .collect_vec();
    // if x == y {
    //     for vv in gg {
    //         let len = vv.len();
    //         let ii: BTreeSet<usize> = (0..len).filter(|&i| vv[i] == x).collect();
    //         // eprintln!("{len} {}", ii.iter().join(","));
    //         for l in 0..vv.len() {
    //             if let Some(&r) = ii.range(l..).next() {
    //                 // eprintln!("{l}-{r}");
    //                 rs += len - r;
    //             }
    //         }
    //     }
    // } else {
    for vv in gg {
        let len = vv.len();
        let ii: BTreeSet<usize> = (0..len).filter(|&i| vv[i] == x).collect();
        let jj: BTreeSet<usize> = (0..len).filter(|&i| vv[i] == y).collect();
        for l in 0..vv.len() {
            if let Some(&ri) = ii.range(l..).next() {
                if let Some(&rj) = jj.range(l..).next() {
                    rs += len - ri.max(rj);
                }
            }
        }
    }
    // }
    println!("{rs}");
}
