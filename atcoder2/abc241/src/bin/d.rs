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
        q: u128,
    };
    let mut bts = BTreeSet::new();
    for i in 0..q {
        input! {t: u8, x: u128};
        match t.cmp(&2) {
            std::cmp::Ordering::Less => {
                bts.insert((x << 32) + i);
            }
            std::cmp::Ordering::Equal => {
                input! {k: usize};
                if let Some(&v) = bts.range(..((x + 1) << 32)).rev().nth(k - 1) {
                    println!("{}", v >> 32);
                } else {
                    println!("-1");
                }
            }
            std::cmp::Ordering::Greater => {
                input! {k: usize};
                if let Some(&v) = bts.range((x << 32)..).nth(k - 1) {
                    println!("{}", v >> 32);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
