#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aa: [isize; n],
    };
    let mut rs = std::isize::MAX;

    // a[2m] を正にする
    // 初項だけ1以上 ほかは 2<=|a|
    let mut sum = 1.max(aa[0]);
    let mut cnt = (aa[0] - sum).abs();
    for (i, &a) in aa[1..].iter().enumerate() {
        if i.is_even() {
            // a[2m+1]
            if sum + a <= -1 {
                sum += a;
            } else {
                let x = sum + a + 1;
                sum = -1;
                cnt += x;
            }
        } else {
            // a[2m]
            if 1 <= sum + a {
                sum += a;
            } else {
                let x = 1 - (sum + a);
                sum = 1;
                cnt += x;
            }
        }
        // eprintln!("{i}: {a} {sum} {cnt}");
    }
    rs = rs.min(cnt);

    // a[2m] を負にする
    // 初項だけ-1以下 ほかは 2<=|a|
    let mut sum = (-1).min(aa[0]);
    let mut cnt = (aa[0] - sum).abs();
    for (i, &a) in aa[1..].iter().enumerate() {
        if i.is_even() {
            // a[2m+1]
            if 1 <= sum + a {
                sum += a;
            } else {
                let x = 1 - (sum + a);
                sum = 1;
                cnt += x;
            }
        } else {
            // a[2m]
            if sum + a <= -1 {
                sum += a;
            } else {
                let x = sum + a + 1;
                sum = -1;
                cnt += x;
            }
        }
        // eprintln!("{i}: {a} {sum} {cnt}");
    }
    rs = rs.min(cnt);
    println!("{rs}");
}
