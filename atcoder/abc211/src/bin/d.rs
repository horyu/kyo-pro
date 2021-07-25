#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }

    let mut d = vec![std::usize::MAX; n];
    d[0] = 0;
    let mut cnt = vec![0; n];
    cnt[0] = 1;

    let mut queue = VecDeque::new();
    queue.push_front(0);
    while let Some(v) = queue.pop_front() {
        for &vv in &g[v] {
            if d[vv] == std::usize::MAX {
                d[vv] = d[v] + 1;
                queue.push_back(vv);
                cnt[vv] = cnt[v];
            } else if d[vv] == d[v] + 1 {
                cnt[vv] += cnt[v];
                cnt[vv] %= 1_000_000_007;
            }
        }
    }
    println!("{}", cnt.last().unwrap());
}
