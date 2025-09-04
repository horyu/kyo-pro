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
    };
    let mut vv = vec![3];
    for _ in 1..n {
        if vv.iter().all_equal() {
            let mut new_vv = vec![1; vv.len() + 1];
            new_vv[0] = 3;
            vv = new_vv;
        } else {
            if let Some(i) = vv.iter().position(|&x| x < 3) {
                vv[i] += 1;
                for j in 1..i {
                    vv[j] = vv[i];
                }
            }
        }
        // eprintln!("{:?}", vv);
    }
    let rs = vv.into_iter().rev().join("");
    println!("{rs}");
}
