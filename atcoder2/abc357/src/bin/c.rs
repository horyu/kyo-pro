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
    let mut x = vec![vec!['#']];
    for _ in 0..n {
        let len = x.len();
        let mut new_x = vec![vec!['.'; len * 3]; len * 3];
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                // [len*i..len*(i+1)][len*j..len*(j+1)]の区間をxで埋める
                for ii in 0..len {
                    for jj in 0..len {
                        new_x[len * i + ii][len * j + jj] = x[ii][jj];
                    }
                }
            }
        }
        x = new_x;
    }
    for x in x {
        let rs = x.iter().join("");
        println!("{rs}");
    }
}
