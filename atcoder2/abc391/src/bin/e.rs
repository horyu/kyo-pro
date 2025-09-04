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
        aa: Bytes,
    };
    let aa = aa.into_iter().map(|x| x - b'0').collect_vec();
    let mut aaa = vec![aa.clone()];
    let mut bb = aa.clone();
    for _ in 0..n {
        bb = bb
            .into_iter()
            .tuples()
            .map(|(x, y, z)| u8::from(2 <= x + y + z))
            .collect_vec();
        aaa.push(bb.clone());
        // eprintln!("{}", bb.iter().join(""));
    }
    if bb[0] == 0 {
        aaa = aaa
            .into_iter()
            .map(|tt| tt.into_iter().map(|x| 1 ^ x).collect_vec())
            .collect_vec();
    }
    let rs = dfs(&aaa, n, 0);
    println!("{rs}");
}

// 0にするコスト
fn dfs(aaa: &[Vec<u8>], i: usize, j: usize) -> usize {
    if aaa[i][j] == 0 {
        return 0;
    }
    if i == 0 {
        return 1;
    }
    let cc = (3 * j..3 * (j + 1))
        .map(|jj| dfs(aaa, i - 1, jj))
        .sorted_unstable()
        .collect_vec();
    cc[0] + cc[1]
}
