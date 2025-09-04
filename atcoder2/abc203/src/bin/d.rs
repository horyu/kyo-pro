#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        k: usize,
        aaa: [[isize; n]; n],
    };
    let nn = n + 1;
    let mut s = vec![vec![0isize; nn]; nn];
    // n=400 k=200
    // 区画 40401 区画のマス数合計 1616040000
    let mut ng = -1;
    let mut ok = 1e9 as isize;
    while ng + 1 < ok {
        let mid = (ng + ok) / 2;
        for i in 0..n {
            for j in 0..n {
                s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j];
                if mid < aaa[i][j] {
                    s[i + 1][j + 1] += 1;
                }
            }
        }
        if (0..=(n - k)).any(|i| {
            (0..=(n - k)).any(|j| {
                s[i + k][j + k] + s[i][j] - s[i + k][j] - s[i][j + k] < (k * k) as isize / 2 + 1
            })
        }) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let rs = ok;
    println!("{rs}");
}
