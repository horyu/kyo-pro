#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
    }
    let mut rs = 0;
    for i in 0..n {
        let mut viewed = vec![false; n];
        dfs(&g, &mut viewed, i);
        rs += viewed.into_iter().filter(|&x| x).count();
    }
    println!("{}", rs);
}

fn dfs(g: &[Vec<usize>], viewed: &mut Vec<bool>, i: usize) {
    if viewed[i] {
        return;
    }
    viewed[i] = true;
    for &next in g[i].iter() {
        dfs(&g, viewed, next);
    }
}
