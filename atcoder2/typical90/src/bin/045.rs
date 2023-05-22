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
        xxyy: [(isize, isize); n],
    };
    // bitDP
    let mut d = vec![vec![0; n]; n];
    for ((i, (ax, ay)), (j, (bx, by))) in xxyy.iter().copied().enumerate().tuple_combinations() {
        d[i][j] = (ax - bx).pow(2) + (ay - by).pow(2);
        d[j][i] = d[i][j];
    }
    let nn = 1 << n;
    let mut cost = vec![0; nn];
    for i in 1..nn {
        for j in 0..n {
            for kk in 0..j {
                if (i >> j) & 1 == 1 && (i >> kk) & 1 == 1 {
                    cost[i] = cost[i].max(d[j][kk]);
                }
            }
        }
    }
    let mut dp = vec![vec![1 << 60; nn]; k + 1];
    dp[0][0] = 0;
    for i in 1..=k {
        for j in 1..nn {
            let mut kk = j;
            while kk != 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - kk].max(cost[kk]));
                kk = (kk - 1) & j;
            }
        }
    }
    let rs = dp[k][nn - 1];
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        k: usize,
        xxyy: [(isize, isize); n],
    };
    let mut used = vec![false; n];
    used[0] = true;
    let rs = dfs(&mut used, &mut vec![0], k - 1, 0, &xxyy);
    println!("{rs}");
}

fn dfs(
    used: &mut Vec<bool>,
    gg: &mut Vec<usize>,
    extra_group: usize,
    max: usize,
    xxyy: &[(isize, isize)],
) -> usize {
    let cal = |ii: &[usize]| -> usize {
        // eprintln!("{}", ii.iter().join("|"));
        ii.iter()
            .copied()
            .map(|i| xxyy[i])
            .tuple_combinations()
            .map(|(a, b)| a.0.abs_diff(b.0).pow(2u32) + a.1.abs_diff(b.1).pow(2u32))
            .max()
            .unwrap_or_default()
    };

    // 未使用点のインデックス
    let ii = used.iter().copied().positions(|tf| !tf).collect_vec();

    if ii.len() <= extra_group {
        return max.max(cal(gg));
    }
    if extra_group == 0 {
        let mut ii = ii;
        ii.extend_from_slice(gg);
        return max.max(cal(&ii));
    }

    // グループを切り直す
    let i = ii[0];
    used[i] = true;
    let mut new_max = max.max(cal(gg));
    new_max = new_max.max(dfs(used, &mut vec![i], extra_group - 1, new_max, xxyy));
    used[i] = false;
    // 現在のグループに追加してDFS
    for i in ii {
        if i < gg.last().copied().unwrap() {
            continue;
        }
        used[i] = true;
        gg.push(i);
        new_max = new_max.min(dfs(used, gg, extra_group, max, xxyy));
        gg.pop();
        used[i] = false;
    }

    new_max
    // dbg!(new_max)
}
