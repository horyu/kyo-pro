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
        k: usize,
        vv: [isize; n],
    };
    let mut rs = 0;
    // 左から i 個取る
    for i in 0..=k.min(n - 1) {
        let xx = vv[..i].iter().copied().collect_vec();
        // 右から j 個取る
        for j in 0..=(k - i) {
            let mut yy = xx.clone();
            yy.extend(vv[i..].iter().copied().rev().take(j));
            yy.sort_unstable();
            yy.reverse();
            // k-i-j 回　負の値を戻す
            for _ in 0..(k - i - j) {
                if let Some(&last) = yy.last() {
                    if last.is_negative() {
                        yy.pop();
                        continue;
                    }
                }
                break;
            }
            rs = rs.max(yy.into_iter().sum());
        }
    }
    println!("{rs}");
}
