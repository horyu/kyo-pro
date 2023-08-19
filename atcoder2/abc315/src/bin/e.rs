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
    };
    let mut g = vec![];
    for i in 0..n {
        input! {
            c: usize,
            pp: [Usize1; c],
        };
        g.push(pp);
    }
    let mut ww = vec![];
    let mut checked = vec![false; n];
    dfs(&mut ww, &g, &mut checked, 0);
    ww.pop();
    let rs = ww.iter().join(" ");
    println!("{rs}");
}

fn dfs(ww: &mut Vec<usize>, g: &Vec<Vec<usize>>, checked: &mut [bool], from: usize) {
    for &to in &g[from] {
        if !checked[to] {
            dfs(ww, g, checked, to);
        }
    }
    checked[from] = true;
    ww.push(from + 1);
}
