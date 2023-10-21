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
        k: usize,
    };
    let max = (n - 1) * (n - 2) / 2;
    if max < k {
        println!("-1");
        return;
    }
    let mut vv = vec![];
    for i in 1..n {
        vv.push((0, i));
    }
    let mut add_cnt = 0;
    for i in 1..n {
        for j in (i + 1)..n {
            if k + add_cnt == max {
                break;
            }
            vv.push((i, j));
            add_cnt += 1;
        }
    }
    println!("{}", vv.len());
    for (a, b) in vv {
        println!("{} {}", a + 1, b + 1);
    }
}
