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
        q: usize,
    };
    // h2s[i] = j; 鳩i が 巣j にいる
    let mut h2s = (0..n).collect_vec();
    // s2p[i] = j; 巣i が 場所j にある
    let mut s2p = (0..n).collect_vec();
    let mut p2s = (0..n).collect_vec();
    for i in 0..q {
        input! {op: i32, a: Usize1};
        if op == 3 {
            // 鳩a のいる場所を出力
            let rs = s2p[h2s[a]] + 1;
            println!("{rs}");
            continue;
        }
        input! {b: Usize1};
        if op == 1 {
            // 鳩a を 場所b の巣に移動
            h2s[a] = p2s[b];
        } else {
            // 場所a と 場所b にある巣を交換
            let sa = p2s[a];
            let sb = p2s[b];
            s2p.swap(sa, sb);
            p2s.swap(a, b);
        }
    }
}
