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
        m: usize,
        k: usize,
        aabb: [(Usize1, Usize1); m],
    };
    // トポロジカルソート
    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        deg[b] += 1;
    }
    let mut perm = vec![!0; n];
    let mut rrss = vec![];
    let mut st = deg.iter().copied().positions(|d| d == 0).collect_vec();
    dfs(0, n, k, &mut rrss, &g, &mut deg, &mut st, &mut perm);
    if rrss.len() == k {
        for rs in rrss {
            println!("{}", rs.iter().map(|&r| r + 1).join(" "));
        }
        return;
    }
    println!("-1");
}

#[allow(clippy::too_many_arguments)]
fn dfs(
    depth: usize,
    n: usize,
    k: usize,
    rrss: &mut Vec<Vec<usize>>,
    g: &[Vec<usize>],
    deg: &mut Vec<usize>,
    st: &mut Vec<usize>,
    perm: &mut Vec<usize>,
) -> bool {
    if depth == n {
        rrss.push(perm.clone());
        return true;
    }
    if st.is_empty() {
        return false;
    }
    for i in (0..st.len()).rev() {
        if rrss.len() == k {
            break;
        }
        let x = st.remove(i);
        for &j in &g[x] {
            deg[j] -= 1;
            if deg[j] == 0 {
                st.push(j);
            }
        }
        perm[depth] = x;
        if !dfs(depth + 1, n, k, rrss, g, deg, st, perm) {
            return false;
        }
        for &j in &g[x] {
            if deg[j] == 0 {
                st.pop();
            }
            deg[j] += 1;
        }
        st.insert(i, x);
    }
    true
}
