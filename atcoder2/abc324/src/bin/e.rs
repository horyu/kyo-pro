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
        t: Chars,
        ss: [Chars; n],
    };
    // sの部分列 が tの先頭何文字目まで含むか
    let ll = ss
        .iter()
        .map(|s| {
            let mut ti = 0;
            let mut cnt = 0;
            for sc in s.iter().copied() {
                if sc == t[ti] {
                    cnt += 1;
                    ti += 1;
                    if ti == t.len() {
                        break;
                    }
                }
            }
            cnt
        })
        .collect_vec();
    // sの部分列 が tの末尾何文字目まで含むか
    let rr = ss
        .iter()
        .map(|s| {
            let mut ti = t.len() - 1;
            let mut cnt = 0;
            for sc in s.iter().copied().rev() {
                if sc == t[ti] {
                    cnt += 1;
                    if ti == 0 {
                        break;
                    }
                    ti -= 1;
                }
            }
            cnt
        })
        .collect_vec();
    let mut ft = ac_library::FenwickTree::new(5e5 as usize + 1, 0usize);
    for r in rr {
        ft.add(r, 1);
    }
    let mut rs = 0usize;
    for l in ll {
        rs += ft.sum((t.len() - l)..);
    }
    println!("{rs}");
}
