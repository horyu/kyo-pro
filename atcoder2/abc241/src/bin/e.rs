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
        aa: [usize; n],
    };
    let mut x_to_i = vec![std::usize::MAX; n];
    let mut xx = vec![0];
    let mut ss = vec![0];
    x_to_i[0] = 0;
    for i in 0..k {
        let s = ss[i] + aa[xx[i] % n];
        let x = s % n;
        xx.push(x);
        ss.push(s);
        eprint!("{x} ");
        if x_to_i[x] != std::usize::MAX {
            // ループ
            let l = x_to_i[x];
            let r = i + 1;
            let loop_size = r - l;
            let loop_count = (k - l) / loop_size;
            let loop_sum = ss[r] - ss[l];
            let rs = ss[l + (k - l) % loop_size] + loop_sum * loop_count;
            println!("{rs}");
            return;
        }
        x_to_i[x] = i + 1;
    }
    let rs = ss.last().unwrap();
    println!("{rs}");
}
