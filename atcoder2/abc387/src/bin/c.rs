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
        l: usize,
        r: usize,
    };
    // https://atcoder.jp/contests/abc387/editorial/11832
    let rs = f(r) - f(l - 1);
    println!("{rs}");
}

fn f(mut v: usize) -> usize {
    let mut dd = vec![];
    while 0 < v {
        dd.push(v % 10);
        v /= 10;
    }
    dd.reverse();
    let n = dd.len();
    let mut rs = 0;
    for i in 1..=n {
        if i == n {
            rs += 1;
            break;
        }
        rs += dd[0].pow((n - 1 - i) as u32) * dd[0].min(dd[i]);
        if dd[0] <= dd[i] {
            break;
        }
    }
    for i in 0..n {
        let max = if i != 0 { 9 } else { dd[0] - 1 };
        for j in 1..=max {
            rs += j.pow((n - 1 - i) as u32);
        }
    }
    rs
}
