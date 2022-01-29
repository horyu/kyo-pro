#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut vvv = vec![vec![]; n];
    for &(a, b) in &aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    let mut rs = vec![0; n];
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    qq.push_back(0);
    pushed[0] = true;
    while let Some(q) = qq.pop_front() {
        for &v in &vvv[q] {
            if !pushed[v] {
                qq.push_back(v);
                pushed[v] = true;
                rs[v] = q;
            }
        }
    }
    let rs = rs[1..].iter().map(|x| x + 1).join("\n");
    println!("Yes\n{rs}");
}
