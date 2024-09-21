#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // dp[i][j] = i(xに訪れた回数 % 2)のときの頂点jにいるときの通り数
    let mut dp0 = vec![vec![ModInt998244353::default(); n]; 2];
    dp0[0][s] += 1;
    for kk in 0..k {
        let mut dp1 = vec![vec![ModInt998244353::default(); n]; 2];
        for (dx, dp) in dp0.iter().enumerate() {
            for (i, d) in dp.iter().copied().enumerate() {
                for j in g[i].iter().copied() {
                    // if 0 < d.val() {
                    //     eprintln!("{kk}-{dx}: {i}->{j}({}) {}+{d}", (dx + usize::from(j == x)) % 2, dp1[(dx + usize::from(j == x)) % 2][j]);
                    // }
                    dp1[(dx + usize::from(j == x)) % 2][j] += d;
                }
            }
        }
        // for (dx, dp) in dp1.iter().enumerate() {
        //     eprintln!("{dx}: {dp:?}");
        // }
        std::mem::swap(&mut dp0, &mut dp1);
    }
    let rs = dp0[0][t];
    println!("{rs}");
}
