#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]
use itertools::Itertools;
// use itertools::Itertools;
use petgraph::graph::UnGraph;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbtt: [(Usize1, Usize1, usize); m]
    };
    // ワーシャルフロイド法
    let mut dist = vec![vec![44850usize * 10000usize; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for (a, b, t) in aabbtt {
        dist[a][b] = t;
        dist[b][a] = t;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let rs = dist
        .into_iter()
        .map(|d| d.into_iter().max().unwrap())
        .min()
        .unwrap();

    println!("{}", rs);
}

// ダイクストラ法
#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        aabbtt: [(Usize1, Usize1, usize); m]
    };
    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabbtt.iter().map(|&(i, j, w)| (nodes[i], nodes[j], w)));
    let rs = nodes
        .iter()
        .map(|&i| {
            petgraph::algo::dijkstra(&g, i, None, |e| *e.weight())
                .into_iter()
                .map(|(_, w)| w)
                .max()
                .unwrap()
        })
        .min()
        .unwrap();
    println!("{}", rs);
}
