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
        q: usize,
    };
    let mut qq = VecDeque::new();
    let mut day = 0usize;
    for _ in 0..q {
        input! {x: u32};
        match x {
            1 => {
                qq.push_back(day);
            }
            2 => {
                input! {t: usize};
                day += t;
            }
            _ => {
                input! {h: usize};
                let mut rs = 0usize;
                while qq.front().is_some_and(|&d| h <= day - d) {
                    qq.pop_front();
                    rs += 1;
                }
                println!("{rs}");
            }
        }
    }
}
