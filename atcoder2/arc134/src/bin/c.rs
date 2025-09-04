#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/arc134/editorial/3328
    let a1 = aa[0];
    let a2n = aa.iter().sum::<usize>() - a1;
    if a1 <= a2n {
        println!("0");
        return;
    }
    let mut rs = ModInt998244353::new(1);
    let div = (1..=(k - 1)).fold(ModInt998244353::new(1), |acc, i| acc / i);
    // (a1 - 1 - a2n) C (k - 1)
    for i in 1..=(k - 1) {
        // rs *= a1 - a2n - 1 - (i - 1);
        rs *= a1 - a2n - i;
    }
    rs *= div;
    // i~2~n (ai + k - 1) C (k - 1)
    for i in 1..n {
        for j in 1..=(k - 1) {
            // rs *= aa[i] + k - 1 - (j - 1);
            rs *= aa[i] + k - j;
        }
        rs *= div;
    }
    println!("{rs}");
}
