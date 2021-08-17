#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

fn main() {
    input! {
        n: usize,
        q: usize,
        aabb: [(Usize1, Usize1); n - 1],
        ccdd: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dis = vec![-1; n];
    dis[0] = 0;
    let mut q = VecDeque::new();
    q.push_back(0usize);
    while let Some(t) = q.pop_front() {
        for &x in &g[t] {
            if dis[x] == -1 {
                dis[x] = dis[t] + 1;
                q.push_back(x);
            }
        }
    }
    for (c, d) in ccdd {
        println!("{}", ["Road", "Town"][(dis[c] % 2 == dis[d] % 2) as usize]);
    }
}
