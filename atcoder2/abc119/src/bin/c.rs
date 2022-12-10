#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        // a: usize,
        // b: usize,
        // c: usize,
        mut abc: [isize; 3],
        mut ll: [isize; n],
    };
    let rs = dfs(n, &abc, &ll, 0, 0, 0, 0);
    println!("{rs}");
}

fn dfs(n: usize, abc: &[isize], ll: &[isize], cur: usize, x: isize, y: isize, z: isize) -> isize {
    if cur == n {
        let xyz = [x, y, z];
        if 0 < xyz.iter().min().copied().unwrap() {
            (0..3).fold(0, |acc, i| acc + (abc[i] - xyz[i]).abs()) - 30
        } else {
            std::isize::MAX >> 4
        }
    } else {
        let l = ll[cur];
        [
            dfs(n, abc, ll, cur + 1, x, y, z),
            dfs(n, abc, ll, cur + 1, x + l, y, z) + 10,
            dfs(n, abc, ll, cur + 1, x, y + l, z) + 10,
            dfs(n, abc, ll, cur + 1, x, y, z + l) + 10,
        ]
        .into_iter()
        .min()
        .unwrap()
    }
}
