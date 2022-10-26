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
        aaa: [[usize; n]; n],
    };
    let mut hhmm = vec![HashMap::new()];
    hhmm[0].insert(aaa[0][0], 1usize);
    // 左上から対角まで
    for size in 2..=n {
        let mut new_hhmm = vec![HashMap::new(); size];
        for (index, hm) in hhmm.into_iter().enumerate() {
            let i = index;
            let j = size - 2 - index;
            let ar = aaa[i][j + 1];
            let ad = aaa[i + 1][j];
            for (k, v) in hm {
                *new_hhmm[index].entry(k ^ ar).or_insert(0) += v;
                *new_hhmm[index + 1].entry(k ^ ad).or_insert(0) += v;
            }
        }
        hhmm = new_hhmm;
    }
    let old_hhmm = hhmm;
    let mut hhmm = vec![HashMap::new()];
    hhmm[0].insert(aaa[n - 1][n - 1], 1usize);
    // 右下から対角まで
    for size in 2..=n {
        let mut new_hhmm = vec![HashMap::new(); size];
        for (index, hm) in hhmm.into_iter().enumerate() {
            let i = n + 1 - size + index;
            let j = n - 1 - index;
            // eprintln!("{size} {index} {i} {j}");
            // 対角要素を2回使わないようにする
            let au = if size == n { 0 } else { aaa[i - 1][j] };
            let al = if size == n { 0 } else { aaa[i][j - 1] };
            for (k, v) in hm {
                *new_hhmm[index].entry(k ^ au).or_insert(0) += v;
                *new_hhmm[index + 1].entry(k ^ al).or_insert(0) += v;
            }
        }
        hhmm = new_hhmm;
    }
    // dbg!(&old_hhmm, &hhmm);
    let mut rs = 0;
    for (i, mut x, mut y) in izip!(0..n, old_hhmm, hhmm) {
        if x.len() < y.len() {
            std::mem::swap(&mut x, &mut y);
        }
        for (k, vx) in x {
            if let Some(&vy) = y.get(&(k)) {
                // eprintln!("{k} {vx} {vy}");
                rs += vx * vy;
            }
        }
    }
    println!("{rs}");
}
