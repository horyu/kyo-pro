#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use counter::Counter;
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
        mut aaa: [[usize; n]; n],
    };
    let f = |aaa: &Vec<Vec<usize>>| {
        let mut bbb: Vec<Vec<Counter<usize, usize>>> = vec![vec![Counter::default(); n]; n];
        bbb[0][0][&aaa[0][0]] += 1usize;
        for i in 0..(n - 1) {
            // eprintln!("{i} {:?}", 0..(n - 1 - i));
            for j in 0..(n - 1 - i) {
                for (k, v) in std::mem::take(&mut bbb[i][j]) {
                    for (ii, jj) in [(i + 1, j), (i, j + 1)] {
                        bbb[ii][jj][&(k ^ aaa[ii][jj])] += v;
                    }
                }
            }
        }
        (0..n)
            .map(|i| std::mem::take(&mut bbb[i][n - 1 - i]))
            .collect_vec()
    };
    let v1 = f(&aaa);
    // 右上から左下への対角線で反転
    for i in 0..(n - 1) {
        for j in 0..(n - 1 - i) {
            aaa[i][j] ^= aaa[n - 1 - j][n - 1 - i];
            aaa[n - 1 - j][n - 1 - i] ^= aaa[i][j];
            aaa[i][j] ^= aaa[n - 1 - j][n - 1 - i];
        }
    }
    let v2 = f(&aaa);
    // dbg!(&v1, &v2);
    let mut rs = 0usize;
    for i in 0..n {
        for (&k1, &v1) in &v1[i] {
            if let Some(v2) = v2[i].get(&(k1 ^ aaa[i][n - 1 - i])) {
                rs += v1 * v2;
                // eprintln!("{i}{}  {k1} {v1} {} {v2}", aaa[i][n - 1 - i], k1 ^ aaa[i][n - 1 - i]);
            }
        }
    }
    println!("{rs}");
}
