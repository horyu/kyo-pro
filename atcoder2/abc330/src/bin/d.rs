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
        ss: [Chars; n],
    };
    // i行目に含まれるoの数
    let gyou = (0..n)
        .map(|i| ss[i].iter().filter(|&&c| c == 'o').count())
        .collect_vec();
    // j列目に含まれるoの数
    let retu = (0..n)
        .map(|j| (0..n).map(|i| ss[i][j]).filter(|&c| c == 'o').count())
        .collect_vec();
    let mut rs = 0;
    for i in 0..n {
        for j in 0..n {
            if ss[i][j] == 'o' {
                rs += (gyou[i] - 1) * (retu[j] - 1);
            }
        }
    }
    println!("{rs}");
}
