#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut d2iijj = vec![vec![]; h + w];
    for i in 0..h {
        for j in 0..w {
            d2iijj[i + j].push((i, j));
        }
    }
    // 貰うDP、壁だったらそこをリセット
    let mut dp = vec![vec![ModInt1000000007::new(0); w]; h];
    let mut yoko = vec![ModInt1000000007::new(0); h];
    let mut tate = vec![ModInt1000000007::new(0); w];
    let mut naname = vec![ModInt1000000007::new(0); h + w];
    dp[0][0] += 1;
    for (i, j) in d2iijj.into_iter().flatten() {
        // 斜め用index
        let k = (h - 1 - i) + j;
        if s[i][j] == '#' {
            yoko[i] *= 0;
            tate[j] *= 0;
            naname[k] *= 0;
            continue;
        }
        dp[i][j] += yoko[i] + tate[j] + naname[k];
        let v = dp[i][j];
        yoko[i] += v;
        tate[j] += v;
        naname[k] += v;
        // eprintln!("{i} {j} {}", dp[i][j]);
    }
    let rs = dp[h - 1][w - 1];
    println!("{rs}");
}
