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
        m: usize,
        uuvv: [(Usize1, Usize1); m],
        pp: [Usize1; 8],
    };
    let mut arr: [usize; 9] = [8; 9];
    for (i, p) in pp.iter().copied().enumerate() {
        arr[p] = i;
    }
    // eprintln!("{arr:?}");
    let mut g = vec![vec![]; 9];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // for (i, vv) in g.iter().enumerate() {
    //     eprintln!("{i} {vv:?}");
    // }
    let mut qq = VecDeque::new();
    qq.push_back((arr, 0));
    let mut pushed = HashSet::new();
    pushed.insert(arr);
    while let Some((qarr, qc)) = qq.pop_front() {
        if qarr.iter().enumerate().all(|(i, &p)| i == p) {
            println!("{qc}");
            return;
        }
        let i = qarr.iter().position(|&p| p == 8).unwrap();
        for j in g[i].iter().copied() {
            let mut arr = qarr;
            arr.swap(i, j);
            if pushed.insert(arr) {
                // eprintln!("{qc}({i}:{j}) {qarr:?}->{arr:?}");
                qq.push_back((arr, qc + 1));
            }
        }
    }
    println!("-1");
}
