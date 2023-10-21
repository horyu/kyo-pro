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
        n: isize,
        m: isize,
        aa: [isize; n],
    };
    let mut hs = HashSet::new();
    let mut add = vec![vec![]; m as usize];
    let mut remove = vec![vec![]; m as usize];
    for (i, &a) in aa.iter().enumerate() {
        if n < a {
            // 最初から無視
            continue;
        }
        if a < 0 {
            // 途中から見る
            let add_i = (-a).div_ceil(i as isize + 1) - 1;
            if add_i < m {
                add[add_i as usize].push(i);
            }
        } else if a < n as isize {
            // 最初から見る
            hs.insert(i);
        }
        let remove_i = (n - a).div_ceil(i as isize + 1) - 1;
        if remove_i < m {
            // そこから無視
            remove[remove_i as usize].push(i);
        }
    }
    // eprintln!("ii:{}", ii.iter().join(" "));
    // for i in 0..(m as usize) {
    //     eprintln!("[{i}]: {} | {}", add[i].iter().join(","), remove[i].iter().join(","));
    // }
    for k in 0..(m as usize) {
        for &r in &remove[k] {
            hs.remove(&r);
        }
        for &a in &add[k] {
            hs.insert(a);
        }
        let values: HashSet<isize> = hs
            .iter()
            .map(|&i| aa[i] + (i as isize + 1) * (k as isize + 1))
            .collect();
        for v in 0..=n {
            if !values.contains(&v) {
                println!("{v}");
                break;
            }
        }
    }
}
