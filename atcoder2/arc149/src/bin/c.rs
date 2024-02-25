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
        n: usize,
    };
    // https://atcoder.jp/contests/arc149/editorial/4911
    let mut rs = vec![vec![0; n]; n];
    match n {
        3 => {
            rs[0] = vec![5, 9, 1];
            rs[1] = vec![3, 7, 8];
            rs[2] = vec![6, 2, 4];
        }
        4 => {
            rs[0] = vec![9, 11, 13, 15];
            rs[1] = vec![1, 3, 5, 7];
            rs[2] = vec![8, 6, 4, 2];
            rs[3] = vec![10, 12, 14, 16];
        }
        5 => {
            rs[0] = vec![5, 7, 11, 13, 17];
            rs[1] = vec![19, 23, 25, 21, 1];
            rs[2] = vec![3, 9, 15, 24, 8];
            rs[3] = vec![6, 12, 18, 2, 4];
            rs[4] = vec![10, 14, 16, 20, 22];
        }
        _ => {
            let nn = n.pow(2u32);
            let mut bb = vec![];
            bb.extend((1..=nn).filter(|&x| x % 2 == 1 && x % 3 != 0));
            bb.extend((1..=nn).filter(|&x| x % 2 == 1 && x % 3 == 0));
            bb.extend((1..=nn).filter(|&x| x % 2 == 0 && x % 3 == 0));
            bb.extend((1..=nn).filter(|&x| x % 2 == 0 && x % 3 != 0));
            for i in 0..n {
                for j in 0..n {
                    rs[i][j] = bb[i * n + j];
                }
            }
        }
    }
    for r in rs {
        println!("{}", r.iter().join(" "));
    }
}
