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
    let mut aaa = vec![];
    for _ in 0..n {
        input! {
            c: usize,
            aa: [usize; c],
        };
        aaa.push(aa);
    }
    input! {
        x: usize,
    };
    let mut min = usize::MAX;
    let mut ii = vec![];
    for i in 0..n {
        if aaa[i].contains(&x) {
            let len = aaa[i].len();
            match len.cmp(&min) {
                std::cmp::Ordering::Less => {
                    min = len;
                    ii = vec![i + 1];
                }
                std::cmp::Ordering::Equal => {
                    ii.push(i + 1);
                }
                std::cmp::Ordering::Greater => (),
            }
        }
    }
    println!("{}", ii.len());
    println!("{}", ii.iter().join(" "));
}
