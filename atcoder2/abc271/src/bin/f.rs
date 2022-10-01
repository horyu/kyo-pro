#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aaa: [[usize; n]; n],
    };
    let mut hhmm = vec![HashMap::new()];
    hhmm[0].insert(aaa[0][0], 1usize);
    // 左上から対角まで
    for size in 2..=n {
        let mut new_hhmm = vec![HashMap::new(); size];
        for (index, hm) in hhmm.into_iter().enumerate() {
            let i = index;
            let j = size - 2 - i;
            let ar = aaa[i][j + 1];
            let ad = aaa[i + 1][j];
            for (k, v) in hm {
                *new_hhmm[index].entry(k ^ ar).or_insert(0) += v;
                *new_hhmm[index + 1].entry(k ^ ad).or_insert(0) += v;
            }
        }
        hhmm = new_hhmm;
    }
    // 対角から右下まで
    for size in (1..=(n - 1)).rev() {
        let mut new_hhmm = vec![HashMap::new(); size];
        for (index, hm) in hhmm.into_iter().enumerate() {
            let i = n - 1 - size + index;
            let j = n - 1 - index;
            if i < n - 1 {
                let ad = aaa[i + 1][j];
                for (&k, &v) in &hm {
                    *new_hhmm[index].entry(k ^ ad).or_insert(0) += v;
                }
            }
            if j < n - 1 {
                let ar = aaa[i][j + 1];
                for (&k, &v) in &hm {
                    *new_hhmm[index - 1].entry(k ^ ar).or_insert(0) += v;
                }
            }
        }
        hhmm = new_hhmm;
    }
    let rs = hhmm[0].get(&0).unwrap_or(&0);
    println!("{rs}");
}
