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
        s: Chars,
    };
    let n = s.len();
    let mut dp = vec![[ModInt1000000007::new(0); 13]; n];
    if s[0] == '?' {
        for j in 0..10 {
            dp[0][j] += 1;
        }
    } else {
        dp[0][(s[0] as u8 - b'0') as usize] += 1;
    }
    for (i, c) in s.into_iter().enumerate().skip(1) {
        if c == '?' {
            for j in 0..13 {
                let tmp = dp[i - 1][j];
                for k in 0..10 {
                    dp[i][(j * 10 + k) % 13] += tmp;
                }
            }
            continue;
        }
        let c = (c as u8 - b'0') as usize;
        for j in 0..13 {
            let tmp = dp[i - 1][j];
            dp[i][(j * 10 + c) % 13] += tmp;
        }
    }
    let rs = dp[n - 1][5];
    println!("{rs}");
}
