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
        t: usize,
        aa: [usize; t],
    };
    let mut mat = vec![vec![!0usize; n]; n];
    for (turn, a) in aa.into_iter().enumerate() {
        let i = (a - 1) / n;
        let j = (a - 1) % n;
        mat[i][j] = turn + 1;
    }
    let mut rs = !0;
    for a in 0..n {
        let max = mat[a].iter().copied().max().unwrap();
        rs = rs.min(max);
        let max = (0..n).map(|b| mat[b][a]).max().unwrap();
        rs = rs.min(max);
    }
    let max = (0..n).map(|a| mat[a][a]).max().unwrap();
    rs = rs.min(max);
    let max = (0..n).map(|a| mat[a][n - 1 - a]).max().unwrap();
    rs = rs.min(max);
    if rs == !0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        t: usize,
        aa: [usize; t],
    };
    // マス(i, j) = N×(i−1)+j が描かれている
    // i行目のマスのビンゴ数
    let mut gyou = vec![0; n];
    // j列目のマスのビンゴ数
    let mut retu = vec![0; n];
    // 斜めのマスのビンゴ数
    let mut naname_migishita = 0;
    let mut naname_hidarashita = 0;
    for (turn, a) in aa.into_iter().enumerate() {
        let i = (a - 1) / n;
        let j = (a - 1) % n;
        // eprintln!("{a} {i} {j}");
        gyou[i] += 1;
        retu[j] += 1;
        if i == j {
            naname_migishita += 1;
        }
        if i + j == n - 1 {
            naname_hidarashita += 1;
        }
        if gyou[i] == n || retu[j] == n || naname_migishita == n || naname_hidarashita == n {
            println!("{}", turn + 1);
            return;
        }
    }
    println!("-1");
}
