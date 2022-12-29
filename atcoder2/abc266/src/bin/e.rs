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
    };
    // f1: (1~6) / 6 = 3.5
    // f2: (4~6) / 6 + 3/6 * f1 = 4.25
    // f3: (5~6) / 6 + 4/6 * f2 = 4.66...
    let mut ff = vec![3.5f64];
    for i in 1..n {
        let last = ff.last().copied().unwrap();
        let kk = (4..=6)
            .filter_map(|k| {
                let k = k as f64;
                if last < k {
                    Some(k)
                } else {
                    None
                }
            })
            .collect_vec();
        let f = (kk.iter().sum::<f64>() + (6 - kk.len()) as f64 * last) / 6.0;
        ff.push(f);
    }
    let rs = ff.last().copied().unwrap();
    println!("{rs}");
}
