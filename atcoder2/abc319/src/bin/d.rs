#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: i128,
        m: i128,
        ll: [i128; n],
    };
    let mut left = ll.iter().copied().max().unwrap() - 1;
    let mut right = 1e17 as i128;
    while 1 < right - left {
        let mid = (left + right) / 2;
        let mut sum = 0;
        let mut cnt = 1;
        for l in ll.iter().copied() {
            if sum == 0 {
                sum = l;
            } else {
                if sum + 1 + l <= mid {
                    sum += 1 + l;
                } else {
                    cnt += 1;
                    sum = l;
                }
            }
        }
        if cnt <= m {
            right = mid;
        } else {
            left = mid;
        }
    }
    let rs = right;
    println!("{rs}");
}
