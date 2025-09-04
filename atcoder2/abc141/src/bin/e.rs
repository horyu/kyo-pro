#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        s: Chars,
    };
    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut rs = 0;
    for i in (0..n).rev() {
        for j in ((i + 1)..n).rev() {
            if s[i] == s[j] {
                dp[i][j] = dp[i][j].max(dp[i + 1][j + 1] + 1);
            }
            rs = rs.max(dp[i][j].min(j - i));
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        s: Chars,
    };
    // https://drken1215.hatenablog.com/entry/2019/09/16/014600
    let mut rs = 0;
    for i in 0..n {
        let lcp = ac_library::z_algorithm_arbitrary(&s[i..]);
        for j in 0..lcp.len() {
            let l = lcp[j].min(j);
            rs = rs.max(l);
        }
    }
    println!("{rs}");
}
