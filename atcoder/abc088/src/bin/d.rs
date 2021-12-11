#![allow(unused_imports)]
use std::collections::VecDeque;

use itertools::Itertools;
// use itertools::Itertools;
use petgraph::{algo::dijkstra, data::Build, graph::UnGraph};
use proconio::{input, marker::*};
fn main() {
    input! {
        h: usize,
        w: usize,
        sss: [Chars; h]
    };
    // https://blog.hamayanhamayan.com/entry/2018/02/18/224521
    // 幅優先探索
    let mut vis = vec![vec![false; 55]; 55];
    let mut dp = vec![vec![0; 55]; 55];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((y, x)) = que.pop_front() {
        if x == w - 1 && y == h - 1 {
            break;
        }
        let mut v = vec![];
        if 0 < x {
            v.push((y, x - 1));
        }
        if x < w - 1 {
            v.push((y, x + 1));
        }
        if 0 < y {
            v.push((y - 1, x));
        }
        if y < h - 1 {
            v.push((y + 1, x));
        }
        for (yy, xx) in v {
            if sss[yy][xx] == '.' && !vis[yy][xx] {
                vis[yy][xx] = true;
                dp[yy][xx] = dp[y][x] + 1;
                que.push_back((yy, xx));
            }
        }
    }
    if dp[h - 1][w - 1] == 0 {
        println!("-1");
    } else {
        let white_cnt = sss
            .iter()
            .map(|ss| ss.iter().filter(|s| **s == '.').count())
            .sum::<usize>();
        println!("{}", white_cnt - dp[h - 1][w - 1] - 1);
    }
}

#[allow(dead_code)]
// ダイクストラ
fn main2() {
    input! {
        h: usize,
        w: usize,
        sss: [Chars; h]
    };
    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = sss
        .iter()
        .map(|ss| {
            ss.iter()
                .map(|s| {
                    if *s == '.' {
                        Some(g.add_node(()))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    for i in 0..h {
        for j in 0..w {
            if let Some(nx) = nodes[i][j] {
                if j < w - 1 {
                    if let Some(ny) = nodes[i][j + 1] {
                        g.add_edge(nx, ny, 1);
                    }
                }
                if i < h - 1 {
                    if let Some(ny) = nodes[i + 1][j] {
                        g.add_edge(nx, ny, 1);
                    }
                }
            }
        }
    }
    let hm = dijkstra(&g, nodes[0][0].unwrap(), nodes[h - 1][w - 1], |e| {
        *e.weight()
    });
    if let Some(move_cnt) = hm.get(&nodes[h - 1][w - 1].unwrap()) {
        let white_cnt = sss
            .iter()
            .map(|ss| ss.iter().filter(|s| **s == '.').count())
            .sum::<usize>();
        println!("{}", white_cnt - move_cnt - 1);
    } else {
        println!("-1");
    }
}
