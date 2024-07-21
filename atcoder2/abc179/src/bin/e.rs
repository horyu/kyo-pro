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
        x: usize,
        m: usize,
    };
    // ダブリング
    let nlog2 = n.ilog2() as usize;
    let mut dp1 = vec![vec![0; m]; nlog2 + 1];
    let mut dp2 = dp1.clone();
    for j in 0..m {
        dp1[0][j] = j * j % m;
        dp2[0][j] = j;
    }
    for i in 1..=nlog2 {
        for j in 0..m {
            dp1[i][j] = dp1[i - 1][dp1[i - 1][j]];
            dp2[i][j] = dp2[i - 1][j] + dp2[i - 1][dp1[i - 1][j]];
        }
    }
    let mut a = x;
    let mut rs = 0;
    for i in 0..=nlog2 {
        if 0 < n & (1 << i) {
            rs += dp2[i][a];
            a = dp1[i][a];
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        x: usize,
        m: usize,
    };
    let mut a = x;
    let mut ss = vec![0];
    let mut a_to_i = vec![!0usize; m];
    for r in 0..n {
        if a_to_i[a] != !0 {
            let l = a_to_i[a];
            let loop_size = r - l;
            let mut rs = ss[l];
            rs += (ss[r] - ss[l]) * ((n - l) / loop_size);
            rs += ss[l + (n - l) % loop_size] - ss[l];
            println!("{rs}");
            return;
        }
        a_to_i[a] = r;
        ss.push(ss.last().unwrap() + a);
        a = a * a % m;
    }
    let rs = ss.last().unwrap();
    println!("{rs}");
}
