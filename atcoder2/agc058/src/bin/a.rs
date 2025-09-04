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
        mut pp: [usize; 2 * n],
    };
    let mut rs = vec![];
    for i in 0..(2 * n - 2) {
        if i.is_even() {
            if pp[i + 1] < pp[i] {
                if pp[i] < pp[i + 2] {
                    // 213
                    rs.push(i + 1);
                    pp.swap(i + 1, i + 2);
                } else {
                    // 312
                    rs.push(i);
                    pp.swap(i, i + 1);
                }
            }
        } else {
            if pp[i] < pp[i + 1] {
                if pp[i] < pp[i + 2] {
                    // 123
                    rs.push(i);
                    pp.swap(i, i + 1);
                } else {
                    // 231
                    rs.push(i + 1);
                    pp.swap(i + 1, i + 2);
                }
            }
        }
    }
    if pp[2 * n - 1] < pp[2 * n - 2] {
        rs.push(2 * n - 2);
        pp.swap(2 * n - 2, 2 * n - 1);
    }
    for i in 0..(n - 1) {
        if i.is_even() {
            if pp[i] > pp[i + 1] {
                panic!("{i}:{}", pp.iter().join(""));
            }
        } else {
            if pp[i] < pp[i + 1] {
                panic!("{}:{}", i, pp.iter().join(""));
            }
        }
    }
    println!("{}", rs.len());
    if !rs.is_empty() {
        println!("{}", rs.into_iter().map(|rs| rs + 1).join(" "));
    }
}
