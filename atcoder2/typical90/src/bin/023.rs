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
        h: usize,
        w: usize,
        ccc: [Chars; h],
    };
    // https://atcoder.jp/contests/typical90/submissions/21987422
    const MOD: usize = 1e9 as usize + 7;
    let mut dp = HashMap::new();
    let mut next = HashMap::new();
    dp.insert(0usize, 1usize);
    for cc in ccc {
        for (j, c) in cc.iter().copied().enumerate() {
            for (x, way) in dp.drain() {
                *next.entry(x >> 1).or_default() += way;
                if c == '.' && x & 1 == 0 {
                    let mut xx = x | 1 | (1 << w);
                    if 0 < j {
                        xx |= 1 << (w - 1);
                    }
                    if j + 1 < w {
                        xx |= (1 << 1) | (1 << (w + 1));
                    }
                    *next.entry(xx >> 1).or_default() += way;
                }
            }
            std::mem::swap(&mut dp, &mut next);
        }
        for w in dp.values_mut() {
            *w %= MOD;
        }
    }
    let rs = dp.values().sum::<usize>() % MOD;
    println!("{rs}");
}
