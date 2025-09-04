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
        m: usize,
        mut s: Chars,
        t: Chars,
    };
    let is_ok =
        |l: usize, s: &[char]| -> bool { izip!(&t, &s[l..]).all(|(&t, &s)| t == s || s == '?') };

    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    let mut cnt = 0;
    for l in 0..=(n - m) {
        if is_ok(l, &s) {
            qq.push_back(l);
            pushed[l] = true;
        }
    }
    while let Some(ql) = qq.pop_front() {
        for i in 0..m {
            if ql + i < n && s[ql + i] != '?' {
                s[ql + i] = '?';
                cnt += 1;
            }
        }
        // 左側
        for l in (ql.saturating_sub(m - 1))..ql {
            if !pushed[l] && is_ok(l, &s) {
                qq.push_front(l);
                for j in l..ql {
                    pushed[j] = true;
                }
            }
        }
        // 右側
        for r in ((ql + 1)..(ql + m).min(n - m + 1)).rev() {
            if !pushed[r] && is_ok(r, &s) {
                qq.push_back(r);
                for j in (ql + 1)..=r {
                    pushed[j] = true;
                }
            }
        }
    }
    let rs = ["No", "Yes"][(n == cnt) as usize];
    println!("{rs}");
}
