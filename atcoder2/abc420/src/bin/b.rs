#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars; n],
    };
    let mut cc = vec![0; n];
    for j in 0..m {
        let mut tmp = [0; 2];
        for i in 0..n {
            tmp[usize::from(ss[i][j] == '1')] += 1;
        }
        if tmp[0] < tmp[1] {
            for i in 0..n {
                if ss[i][j] == '0' {
                    cc[i] += 1;
                }
            }
        } else {
            for i in 0..n {
                if ss[i][j] == '1' {
                    cc[i] += 1;
                }
            }
        }
    }
    let max = *cc.iter().max().unwrap();
    let rs = cc
        .into_iter()
        .positions(|x| x == max)
        .map(|i| i + 1)
        .join(" ");
    println!("{rs}");
}
