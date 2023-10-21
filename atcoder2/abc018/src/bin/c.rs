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
        r: usize,
        c: usize,
        k: usize,
        ss: [Chars; r],
    };
    let mut a = ss
        .iter()
        .map(|s| s.iter().copied().map(|c| c == 'o').collect_vec())
        .collect_vec();
    for kk in 1..k {
        let mut b = vec![vec![false; c]; r];
        for i in kk..(r.saturating_sub(kk)) {
            for j in kk..(c.saturating_sub(kk)) {
                if a[i][j] && a[i - 1][j] && a[i + 1][j] && a[i][j - 1] && a[i][j + 1] {
                    b[i][j] = true;
                }
            }
        }
        a = b;
        // eprintln!("{kk}");
        // for aa in &a {
        //     eprintln!("{}", aa.iter().map(|&a| ['F', 'T'][a as usize]).join(" "));
        // }
    }
    let rs = a.into_iter().flatten().filter(|&tf| tf).count();
    println!("{rs}");
}
