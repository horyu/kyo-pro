#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    // 木の直径
    let bfs = |mut pos: usize| -> (usize, usize) {
        let mut max = 0;
        let mut qq = VecDeque::new();
        qq.push_back((!0, pos, 0));
        while let Some((from, to, cost)) = qq.pop_front() {
            if max < cost {
                max = cost;
                pos = to;
            }
            for &v in &vvv[to] {
                if v != from {
                    qq.push_back((to, v, cost + 1));
                }
            }
        }
        (max, pos)
    };
    let tmp = bfs(0);
    let tmp = bfs(tmp.1);
    println!("{}", tmp.0 + 1);
}
