#![allow(unused_imports)]
use std::collections::{HashMap, VecDeque};
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut gg = vec![vec![]; n];
    for &(a, b) in &aabb {
        gg[a].push(b);
        gg[b].push(a);
    }
    let mut rs = HashMap::new();
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    qq.push_back(0usize);
    pushed[0] = true;
    let mut pre_cc = vec![0; n];
    while !qq.is_empty() {
        let mut next_qq = VecDeque::new();
        while let Some(q) = qq.pop_front() {
            let mut c = 1;
            for &g in &gg[q] {
                if pushed[g] {
                    continue;
                }
                if c == pre_cc[q] {
                    c += 1;
                }
                rs.insert((q, g), c);
                rs.insert((g, q), c);
                pre_cc[g] = c;

                next_qq.push_back(g);
                pushed[g] = true;
                c += 1;
            }
        }
        qq = next_qq;
    }

    println!("{}", pre_cc.into_iter().max().unwrap());
    for ab in aabb {
        println!("{}", rs.get(&ab).unwrap());
    }
}
