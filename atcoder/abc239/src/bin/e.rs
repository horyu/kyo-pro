#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        xx: [usize; n],
        aabb: [(Usize1, Usize1); n - 1],
        vvkk:[(Usize1, Usize1); q],
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    let mut pp = vec![std::usize::MAX; n];
    let mut ccc = vec![vec![]; n];
    let mut qq = VecDeque::new();
    qq.push_back((std::usize::MAX, 0));
    while let Some((p, q)) = qq.pop_front() {
        for &v in &vvv[q] {
            if p != v {
                qq.push_back((q, v));
                pp[v] = q;
                ccc[q].push(v);
            }
        }
    }

    let mut mmm = vec![vec![]; n];
    let mut children_viewed_counts = vec![0; n];
    let mut qq = VecDeque::new();
    // 葉を最初見ていく
    for i in 0..n {
        if ccc[i].is_empty() {
            mmm[i] = vec![xx[i]];

            let p = pp[i];
            children_viewed_counts[p] += 1;
            if children_viewed_counts[p] == ccc[p].len() {
                qq.push_back(p);
            }
        }
    }
    while let Some(q) = qq.pop_front() {
        let mut mm = vec![xx[q]];
        for &c in &ccc[q] {
            mm.extend_from_slice(&mmm[c]);
        }
        mm.sort_unstable_by_key(|&m| std::cmp::Reverse(m));
        mm.truncate(20);
        mmm[q] = mm;

        if q != 0 {
            let p = pp[q];
            children_viewed_counts[p] += 1;
            if children_viewed_counts[p] == ccc[p].len() {
                qq.push_back(p);
            }
        }
    }
    for (v, k) in vvkk {
        println!("{}", mmm[v][k]);
    }
}
