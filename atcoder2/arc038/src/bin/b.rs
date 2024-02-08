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
        h: usize,
        w: usize,
        sss: [Chars; h],
    };
    let sss = sss
        .into_iter()
        .map(|ss| ss.into_iter().map(|c| c == '.').collect_vec())
        .collect_vec();
    let mut memo = vec![vec![None::<bool>; w]; h];
    let rs = ["Second", "First"][f(&mut memo, &sss, 0, 0) as usize];
    println!("{rs}");
}

fn f(memo: &mut Vec<Vec<Option<bool>>>, sss: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let h = sss.len();
    let w = sss[0].len();
    if i == h || j == w || !sss[i][j] {
        return true;
    }
    if let Some(b) = memo[i][j] {
        return b;
    }
    let b = !f(memo, sss, i + 1, j) || !f(memo, sss, i, j + 1) || !f(memo, sss, i + 1, j + 1);
    memo[i][j] = Some(b);
    b
}
