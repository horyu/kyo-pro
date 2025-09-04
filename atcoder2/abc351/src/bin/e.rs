#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
    };
    // xxyy を 偶数・奇数で分ける
    let mut rs = 0isize;
    for (_, xxyy) in xxyy.into_iter().into_group_map_by(|&(x, y)| (x + y) % 2) {
        // eprintln!("{xxyy:?}");
        let tyousei = (xxyy[0].0 + xxyy[0].1) % 2;
        // (x, y) -> (x + y, x - y) に変換
        let ppqq = xxyy
            .into_iter()
            .map(|(x, y)| ((x + y + tyousei) / 2, (x - y + tyousei) / 2))
            .collect_vec();
        // eprintln!("{ppqq:?}");
        // 頂点群のマンハッタン距離の総和を求める
        // まず全ての頂点群に対してX座標のみで考える
        let pp = ppqq.iter().map(|&(p, _)| p).sorted_unstable().collect_vec();
        let mut sum = pp.iter().map(|&p| p - pp[0]).sum::<isize>();
        rs += sum;
        for i in 1..pp.len() {
            let d = pp[i] - pp[i - 1];
            sum -= d + d * (pp.len() - 1 - i) as isize;
            rs += sum;
        }
        // 次に全ての頂点群に対してY座標のみで考える
        let qq = ppqq.iter().map(|&(_, q)| q).sorted_unstable().collect_vec();
        let mut sum = qq.iter().map(|&q| q - qq[0]).sum::<isize>();
        rs += sum;
        for i in 1..qq.len() {
            let d = qq[i] - qq[i - 1];
            sum -= d + d * (qq.len() - 1 - i) as isize;
            rs += sum;
        }
    }
    println!("{rs}");
}
