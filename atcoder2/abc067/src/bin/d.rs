#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
#![feature(drain_filter)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    // 1とnの真ん中n寄りで切る
    let mut ff = vec![!0; n];
    let mut qq = VecDeque::new();
    qq.push_back((0, !0));
    while let Some((to, from)) = qq.pop_front() {
        for &next in &g[to] {
            if next == from {
                continue;
            }
            ff[next] = to;
            qq.push_back((next, to));
        }
    }
    let mut vv = vec![n - 1];
    while *vv.last().unwrap() != 0 {
        vv.push(ff[*vv.last().unwrap()]);
    }
    // eprintln!("{}", vv.iter().join(" "));

    let cut_l = vv[(vv.len() - 2) / 2];
    let cut_r = vv[(vv.len() - 2) / 2 + 1];
    // dbg!(cut_l, cut_r);
    g[cut_l].drain_filter(|x| *x == cut_r);
    g[cut_r].drain_filter(|x| *x == cut_l);
    let calc = |start: usize| -> usize {
        let mut cnt = 0;
        let mut qq = VecDeque::new();
        qq.push_back((start, start));
        while let Some((to, from)) = qq.pop_front() {
            for &next in &g[to] {
                if next == from {
                    continue;
                }
                cnt += 1;
                qq.push_back((next, to));
            }
        }
        // dbg!(start, cnt);
        cnt
    };
    let rs = if calc(n - 1) < calc(0) {
        "Fennec"
    } else {
        "Snuke"
    };
    println!("{rs}");
}
