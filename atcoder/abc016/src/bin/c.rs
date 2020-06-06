#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut g = vec![Vec::with_capacity(n); n];
    for (a, b) in aabb {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        let mut directs: HashSet<_> = (&g[i]).iter().collect();
        let indirects: HashSet<_> = directs.iter().flat_map(|&&j| &g[j]).collect();
        directs.insert(&i);
        println!("{}", indirects.difference(&directs).count());
    }
}
