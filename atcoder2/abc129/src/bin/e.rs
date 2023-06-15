#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        ll: Chars,
    };
    // 桁DP https://blog.hamayanhamayan.com/entry/2019/06/10/214119
    let n = ll.len();
    let mut dp = vec![[ModInt1000000007::default(); 2]; n + 1];
    dp[0][0] += 1;
    for dgt in 0..n {
        for is_less in 0..2 {
            if ll[dgt] == '1' {
                dp[dgt + 1][1] = dp[dgt + 1][1] + dp[dgt][is_less];
            } else {
                dp[dgt + 1][is_less] = dp[dgt + 1][is_less] + dp[dgt][is_less];
            }

            if ll[dgt] == '1' || is_less == 1 {
                dp[dgt + 1][is_less] = dp[dgt + 1][is_less] + dp[dgt][is_less] * 2;
            }
        }
    }
    // for i in 0..=n {
    //     eprintln!("{i}: {} {}", dp[i][0], dp[i][1]);
    // }
    let rs = dp[n][0] + dp[n][1];
    println!("{rs}");
}
