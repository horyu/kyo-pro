#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    };

    let blocks = s
        .into_iter()
        .group_by(|c| *c == 'L')
        .into_iter()
        .map(|(_c, block)| block.count())
        .collect_vec();
    let m = blocks.len();

    let rs = if m / 2 <= k {
        // 全部同じ向き
        n - 1
    } else {
        // １グループを反転させると2人増える
        // それぞれのグループの幸福な人数 + 反転可能回数 * 2
        blocks.iter().map(|c| c - 1).sum::<usize>() + k * 2
    };
    println!("{}", rs);
}
