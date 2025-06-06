#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        mut cc: [usize; n - 1],
        mut aa: [usize; n - 1],
    };
    cc.insert(0, 0);
    aa.insert(0, 1); // ダミー
    let ii = aa.iter().copied().positions(|a| 0 < a).collect_vec();
    let mut vv = vec![!0usize; n];
    vv[ii[ii.len() - 1]] = 0;
    // 右端から一番近い中身入り茶碗に移していく
    for (r, l) in ii.iter().copied().rev().tuple_windows() {
        for j in (l..=r).rev() {
            let c = cc[j];
            if j.saturating_sub(c) <= l {
                vv[l] = vv[j] + 1;
                break;
            }
            for jj in (j - c)..j {
                vv[jj] = vv[jj].min(vv[j] + 1);
            }
        }
    }
    let rs = vv[0];
    println!("{rs}");
}
