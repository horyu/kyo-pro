#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    // 同じ長さになるように aa, bb から要素を取り除く（取り除いた要素数x）
    // aa, bb の同じインデックスで異なる要素数y
    // x + y が最小になるようにする

    let (n, m, aa, bb) = if n <= m {
        (n, m, aa, bb)
    } else {
        (m, n, bb, aa)
    };

    // dp[i][j] = aa[..i] と bb[..j] までマッチングさせたときの x + y の最小値
    let mut dp = vec![vec![!0; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            let pre = dp[i][j];
            if j < m {
                dp[i][j + 1] = dp[i][j + 1].min(pre + 1);
            }
            if i < n {
                dp[i + 1][j] = dp[i + 1][j].min(pre + 1);
            }
            if i < n && j < m {
                let d = usize::from(aa[i] != bb[j]);
                dp[i + 1][j + 1] = dp[i + 1][j + 1].min(pre + d);
            }
        }
    }
    // for (i, dp) in dp.iter().enumerate() {
    //     eprintln!("{i}: {:?}", dp);
    // }
    let rs = dp[n][m];
    println!("{rs}");
}
