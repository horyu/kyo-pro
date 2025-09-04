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
        aa: [usize; n],
    };
    if n == 1 {
        println!("0");
        return;
    }
    // cc[i] = i日間割当可能日があるときの最大の生産量（0,i+1日目を休みとする）
    let mut cc = vec![0; n + 1];
    cc[1] = aa[0];
    cc[2] = aa[0] * 2;
    for i in 3..n {
        if i % 2 == 1 {
            cc[i] = cc[i - 1] + aa[i / 2];
        } else {
            cc[i] = cc[i - 1] + aa[(i - 1) / 2];
        }
    }
    for i in 3..n {
        for j in 1..=(i / 2) {
            cc[i] = cc[i].max(cc[j] + cc[i - 1 - j]);
        }
    }
    // eprintln!("{}", cc.iter().join(" "));
    let rs = cc[n - 1];
    println!("{rs}");
}
