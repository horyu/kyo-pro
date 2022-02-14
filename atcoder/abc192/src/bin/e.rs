#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use proconio::{input, marker::*};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        aabbttkk: [(Usize1, Usize1, usize, usize); m]
    };
    let mut vvv = vec![vec![]; n];
    for &(a, b, t, k) in &aabbttkk {
        vvv[a].push((b, t, k));
        vvv[b].push((a, t, k));
    }

    // ダイクストラ
    // https://theory-and-me.hatenablog.com/entry/2019/09/08/182442
    let mut dist = vec![std::usize::MAX >> 2; n];
    let mut qq = BinaryHeap::new();
    dist[x] = 0;
    qq.push((std::cmp::Reverse(0), x));
    while let Some((std::cmp::Reverse(a_cost), a)) = qq.pop() {
        if dist[a] < a_cost {
            continue;
        }
        for &(b, t, k) in &vvv[a] {
            let b_cost = dist[a].div_ceil(k) * k + t;
            if b_cost < dist[b] {
                dist[b] = b_cost;
                qq.push((std::cmp::Reverse(b_cost), b));
            }
        }
    }
    if dist[y] == std::usize::MAX >> 2 {
        println!("-1");
    } else {
        println!("{}", dist[y]);
    }
}
