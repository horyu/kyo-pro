#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1]
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
    let mut range_sizes = vec![1; n];
    let mut children_viewed_counts = vec![0; n];
    let mut qq = VecDeque::new();
    for i in 0..n {
        if ccc[i].is_empty() {
            let p = pp[i];
            children_viewed_counts[p] += 1;
            if children_viewed_counts[p] == ccc[p].len() {
                qq.push_back(p);
            }
        }
    }
    while let Some(q) = qq.pop_front() {
        if ccc[q].len() == 1 {
            range_sizes[q] = range_sizes[ccc[q][0]];
        } else {
            range_sizes[q] = ccc[q].iter().map(|&c| range_sizes[c]).sum::<usize>();
        }
        if q != 0 {
            let p = pp[q];
            children_viewed_counts[p] += 1;
            if children_viewed_counts[p] == ccc[p].len() {
                qq.push_back(p);
            }
        }
    }

    let mut rs = vec![(0, 0); n];
    dfs(&mut rs, &ccc, &range_sizes, 0, 0);
    for (l, r) in rs {
        println!("{l} {r}");
    }
}

fn dfs(
    rs: &mut Vec<(usize, usize)>,
    ccc: &[Vec<usize>],
    range_sizes: &[usize],
    i: usize,
    offset: usize,
) {
    match ccc[i].len() {
        0 => {
            rs[i] = (offset + 1, offset + 1);
        }
        1 => {
            dfs(rs, ccc, range_sizes, ccc[i][0], offset);
            rs[i] = rs[ccc[i][0]];
        }
        _ => {
            let mut offset = offset;
            for &c in &ccc[i] {
                dfs(rs, ccc, range_sizes, c, offset);
                offset += range_sizes[c];
            }
            rs[i].0 = rs[ccc[i][0]].0;
            rs[i].1 = rs[*ccc[i].last().unwrap()].1;
        }
    };
}
