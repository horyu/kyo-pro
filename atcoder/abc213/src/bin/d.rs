#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    vec,
};

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    for v in g.iter_mut() {
        v.sort_unstable();
    }
    let mut ans = vec![];
    dfs(&mut ans, &g, 0, std::usize::MAX);
    println!("{}", ans.into_iter().map(|i| (i + 1).to_string()).join(" "));
}

fn dfs(ans: &mut Vec<usize>, g: &Vec<Vec<usize>>, crr: usize, pre: usize) {
    ans.push(crr);
    for &nxt in &g[crr] {
        if nxt != pre {
            dfs(ans, g, nxt, crr);
            ans.push(crr);
        }
    }
}
