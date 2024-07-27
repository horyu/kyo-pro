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
        k: usize,
        s: Chars,
    };
    // dp[i][j] = k+i文字目まで見て、末尾k文字がjであるような文字列の個数
    let mut dp = vec![vec![ModInt998244353::default(); 1 << k]; n + 1 - k];
    // ttt = ['A', 'B']^k
    let ttt = (0..k)
        .map(|_| ['A', 'B'])
        .multi_cartesian_product()
        .collect_vec();
    let mut ngs = HashSet::new();
    for (i, tt) in ttt.iter().enumerate() {
        if izip!(tt.iter(), tt.iter().rev()).all(|(x, y)| x == y) {
            ngs.insert(i);
            continue;
        }
        if izip!(s.iter().copied(), tt.iter().copied()).all(|(sc, tc)| sc == '?' || sc == tc) {
            dp[0][i] = 1.into();
        }
    }
    for i in k..n {
        for (j, tt) in ttt.iter().enumerate() {
            if ngs.contains(&j) {
                continue;
            }
            // A:0 B:1 として捉える
            let sscc = if s[i] == '?' {
                vec![0, 1]
            } else {
                vec![s[i] as u8 - b'A']
            };
            for sc in sscc {
                // jの下kビットを左に1ビットシフトしてscを追加
                let jj = (j << 1 | sc as usize) % (1 << k);
                if ngs.contains(&jj) {
                    continue;
                }
                let tmp = dp[i - k][j];
                dp[i - k + 1][jj] += tmp;
            }
        }
    }
    let rs = dp[n - k].iter().sum::<ModInt998244353>();
    println!("{rs}");
}
