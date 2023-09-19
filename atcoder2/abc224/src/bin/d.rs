#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(map_try_insert)]
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
    let mut g = vec![vec![]; 9];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // 駒0~7, 空白8
    let mut arr = [8; 9];
    for (i, &p) in pp.iter().enumerate() {
        arr[p] = i;
    }
    let mut qq = VecDeque::new();
    qq.push_back((arr, 0));
    let mut pushed = HashSet::new();
    pushed.insert(arr);
    while let Some((mut arr, cnt)) = qq.pop_front() {
        if arr.iter().enumerate().all(|(i, &p)| p == i) {
            println!("{cnt}");
            return;
        }
        let empty_pos = arr.iter().position(|&p| p == 8).unwrap();
        for swap_pos in g[empty_pos].iter().copied() {
            arr.swap(empty_pos, swap_pos);
            if pushed.insert(arr) {
                qq.push_back((arr, cnt + 1));
            }
            arr.swap(empty_pos, swap_pos);
        }
    }
    println!("-1");
}
