#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        n: usize,
        s: Chars,
    };
    let vv = s
        .iter()
        .copied()
        .filter_map(|c| match c {
            'a' => Some(0),
            't' => Some(1),
            'c' => Some(2),
            'o' => Some(3),
            'd' => Some(4),
            'e' => Some(5),
            'r' => Some(6),
            _ => None,
        })
        .collect_vec();
    let mut dp = [ModInt1000000007::default(); 9];
    for v in vv {
        if v == 0 {
            dp[v] += 1;
        } else {
            dp[v] += dp[v - 1];
        }
    }
    let rs = dp[6];
    println!("{rs}");
}
