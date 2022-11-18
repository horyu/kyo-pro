#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut btm = BTreeMap::new();
    for &a in &aa {
        *btm.entry(a).or_insert(0) += 1usize;
    }
    // 初期配置何個以下を食べ終えたか
    let mut eaten_size = 0usize;
    // 1周で何個食べるか
    let mut loop_eat_num = aa.iter().filter(|&&a| eaten_size < a).count();
    // 食べた送料
    let mut sum = 0usize;
    while sum + loop_eat_num < k {
        // 何周できるか = min(kに達するまでに必要な周回数, 0個に最も近いりんごの数)
        let (&next_size, &next_cnt) = btm.range((eaten_size + 1)..).next().unwrap();
        let loop_cnt = ((k - sum) / loop_eat_num).min(next_size - eaten_size);
        sum += loop_cnt * loop_eat_num;
        // dbg!(next_size, sum, loop_cnt);
        eaten_size += loop_cnt;
        loop_eat_num -= next_cnt;
    }
    let mut nokori = k - sum;
    // dbg!(eaten_size, loop_eat_num, sum, nokori);
    let mut rs = vec![];
    for a in aa {
        let mut b = a.saturating_sub(eaten_size);
        if 0 < nokori && 0 < b {
            nokori -= 1;
            b -= 1;
        }
        rs.push(b);
    }
    let rs = rs.into_iter().join(" ");
    println!("{rs}");
}
