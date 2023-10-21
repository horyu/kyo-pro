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
        s: Bytes
    };
    let s = s.into_iter().map(|s| (s - b'0') as usize).collect_vec();
    let n = s.len();
    if n == 1 {
        println!("{}", s[0]);
        return;
    }
    // 123
    // 12+3
    // 1+23
    // 1+2+3
    // 1*(1*100 + 1*10 + 2*1)
    // 2*(        2*10 + 2*1)
    // 3*(               4*1)

    // 1234
    // 123+4
    // 12+34
    // 12+3+4
    // 1+234
    // 1+23+4
    // 1+2+34
    // 1+2+3+4
    // 1*(1*1000 + 1*100 + 2*10 + 4*1)
    // 2*(         2*100 + 2*10 + 4*1)
    // 3*(                 4*10 + 4*1)
    // 4*(                        8*1)
    // 下から計算していく
    let pow10 = (0..n)
        .map(|exp| ModInt998244353::new(10).pow(exp as u64))
        .collect_vec();
    let pow2 = (0..n)
        .map(|exp| ModInt998244353::new(2).pow(exp as u64))
        .collect_vec();
    let mut rs = ModInt998244353::new(0);
    let mut mul = pow2[n - 1] * pow10[0]; // 8*1
    rs += mul * s[n - 1]; // 8*1 * 4
    for i in (0..(n - 1)).rev() {
        // 直前の先頭の項を半分にする 8*1 → 4*1
        mul -= pow2[i] * pow10[n - 2 - i];
        // 4*1 → 4*10 + 4*1
        mul += pow2[i] * pow10[n - 1 - i];
        rs += mul * s[i];
    }
    println!("{rs}");
}
