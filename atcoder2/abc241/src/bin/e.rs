#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut x_to_i = vec![std::usize::MAX; n];
    let mut x = 0usize;
    x_to_i[x] = 0;
    let mut ss = vec![0usize];
    for r in 1..=k {
        let new_s = ss.last().unwrap() + aa[x];
        ss.push(new_s);
        x = new_s % n;
        if x_to_i[x] != std::usize::MAX {
            // ループ
            let l = x_to_i[x];
            let loop_size = r - l;
            let mut rs = ss[l];
            rs += (ss[r] - ss[l]) * ((k - l) / loop_size);
            rs += ss[l + (k - l) % loop_size] - ss[l];
            println!("{rs}");
            return;
        }
        x_to_i[x] = r;
    }
    println!("{}", ss.last().unwrap());
}
