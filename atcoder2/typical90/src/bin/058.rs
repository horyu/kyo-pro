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
        k: usize,
    };
    const MAX: usize = 1e5 as usize;
    let mut x_to_i = vec![std::usize::MAX; MAX];
    let mut x = n;
    x_to_i[x] = 0;
    let mut i_to_x = vec![x];
    for r in 1..=k {
        x = ((0..5).map(|a| x / 10usize.pow(a) % 10).sum::<usize>() + x) % MAX;
        i_to_x.push(x);
        if x_to_i[x] != std::usize::MAX {
            // ループ
            let l = x_to_i[x];
            let loop_size = r - l;
            let rs = i_to_x[l + (k - l) % loop_size];
            println!("{rs}");
            return;
        }
        x_to_i[x] = r;
    }
    let rs = x;
    println!("{rs}");
}
