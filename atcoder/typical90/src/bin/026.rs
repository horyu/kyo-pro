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
    // 根からの距離の偶奇で分けて、個数がn/2以上の方から先頭n/2個選択する
    let mut qq = VecDeque::new();
    let mut eeoo = vec![vec![]; 2];
    qq.push_back((!0, 0, 0));
    while let Some((from, to, d)) = qq.pop_front() {
        eeoo[d % 2].push(to);
        for &v in &vvv[to] {
            if v != from {
                qq.push_back((to, v, d + 1));
            }
        }
    }
    let rs = eeoo
        .into_iter()
        .max_by_key(|vv| vv.len())
        .unwrap()
        .into_iter()
        .take(n / 2)
        .map(|v| v + 1)
        .join(" ");
    println!("{rs}");
}
