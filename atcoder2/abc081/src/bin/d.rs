#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n],
    };
    let mut rs = vec![];
    if let itertools::MinMaxResult::MinMax(&min, &max) = aa.iter().minmax() {
        if min == 0 || min.abs() <= max.abs() {
            // 正整数を足していく
            let mut j = aa.iter().position_max().unwrap();
            for i in 0..(n - 1) {
                while aa[i + 1] < aa[i] {
                    aa[i + 1] += aa[j];
                    rs.push((j + 1, i + 2));
                    j = aa.iter().position_max().unwrap();
                }
            }
        } else {
            // 負整数を足していく
            let mut j = aa.iter().position_min().unwrap();
            for i in (0..(n - 1)).rev() {
                while aa[i + 1] < aa[i] {
                    aa[i] += aa[j];
                    rs.push((j + 1, i + 1));
                    j = aa.iter().position_min().unwrap();
                }
            }
        }
    }
    assert!(rs.len() <= 2 * n, "{}", rs.len());
    println!("{}", rs.len());
    for (x, y) in rs {
        println!("{x} {y}");
    }
    // eprintln!("{}", aa.iter().join(" "));
}
