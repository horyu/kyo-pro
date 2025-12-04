#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut rerooting = Rerooting::new(n);
    for (a, b) in aabb {
        rerooting.add_edge(a, b);
        rerooting.add_edge(b, a);
    }
    rerooting.build();
    // TODO: 再確認
    // println!("{rs}");
}

// https://algo-logic.info/tree-dp/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DP {
    val: usize,
    idx: Option<usize>,
}
impl DP {
    fn new(val: usize, idx: Option<usize>) -> Self {
        Self { val, idx }
    }
    fn identity() -> Self {
        Self { val: 0, idx: None }
    }
}

fn merge(db_cum: &DP, d: &DP) -> DP {
    *db_cum.max(d)
}
fn add_root(d: &DP) -> DP {
    DP::new(d.val + 1, d.idx)
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    to: usize,
}

struct Rerooting {
    dp: Vec<Vec<DP>>,
    ans: Vec<DP>,
    g: Vec<Vec<Edge>>,
}

impl Rerooting {
    fn new(n: usize) -> Self {
        Self {
            dp: vec![vec![]; n],
            ans: vec![DP::identity(); n],
            g: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(Edge { to: v });
        self.g[v].push(Edge { to: u });
    }

    fn build(&mut self) {
        self.dfs(0, !0);
        self.bfs(0, &DP::identity(), !0);
    }

    fn dfs(&mut self, v: usize, p: usize) -> DP {
        let mut dp_cum = DP::identity();
        let deg = self.g[v].len();
        self.dp[v] = vec![DP::identity(); deg];
        for i in 0..deg {
            let u = self.g[v][i].to;
            if u == p {
                continue;
            }
            self.dp[v][i] = self.dfs(u, v);
            dp_cum = merge(&dp_cum, &self.dp[v][i]);
        }
        add_root(&dp_cum)
    }
    fn bfs(&mut self, v: usize, dp_p: &DP, p: usize) {
        let deg = self.g[v].len();
        for i in 0..deg {
            if self.g[v][i].to == p {
                self.dp[v][i] = *dp_p;
            }
        }
        let mut dp_l = vec![DP::identity(); deg + 1];
        let mut dp_r = dp_l.clone();
        for i in 0..deg {
            dp_l[i + 1] = merge(&dp_l[i], &self.dp[v][i]);
        }
        for i in (0..deg).rev() {
            dp_r[i] = merge(&dp_r[i + 1], &self.dp[v][i]);
        }

        self.ans[v] = add_root(&dp_l[deg]);
        for i in 0..deg {
            let u = self.g[v][i].to;
            if u == p {
                continue;
            }
            self.bfs(u, &add_root(&merge(&dp_l[i], &dp_r[i + 1])), v);
        }
    }
}
