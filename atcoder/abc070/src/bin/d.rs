#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::graph::UnGraph;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aabbcc: [(Usize1, Usize1, usize); n - 1],
        q: usize,
        k: Usize1,
        xxyy: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    let mut dd = vec![0; n];
    for (a, b, c) in aabbcc {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    #[allow(clippy::ptr_arg)]
    fn dfs(v: usize, p: usize, d: usize, g: &Vec<Vec<(usize, usize)>>, dd: &mut Vec<usize>) {
        dd[v] = d;
        for &e in &g[v] {
            if e.0 == p {
                continue;
            }
            dfs(e.0, v, d + e.1, g, dd);
        }
    }

    dfs(k, std::usize::MAX, 0, &g, &mut dd);

    for (x, y) in xxyy {
        println!("{}", dd[x] + dd[y]);
    }
}

// ダイクストラ法
#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        aabbcc: [(Usize1, Usize1, usize); n - 1],
        q: usize,
        k: Usize1,
        xxyy: [(Usize1, Usize1); q],
    };
    let mut g = UnGraph::<(), usize>::new_undirected();
    let nodes = (0..n).map(|_| g.add_node(())).collect_vec();
    g.extend_with_edges(aabbcc.iter().map(|&(i, j, w)| (nodes[i], nodes[j], w)));

    let hm = petgraph::algo::dijkstra(&g, nodes[k], None, |e| *e.weight());
    for (x, y) in xxyy {
        let rs = hm.get(&nodes[x]).unwrap() + hm.get(&nodes[y]).unwrap();
        println!("{}", rs);
    }
}
