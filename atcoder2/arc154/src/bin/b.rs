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
        s: Bytes,
        t: Bytes,
    };
    // https://atcoder.jp/contests/arc154/editorial/5533
    if s.iter().counts() != t.iter().counts() {
        println!("-1");
        return;
    }
    // 二分探索
    let mut l = 0;
    let mut r = n - 1;
    while 1 < r - l {
        let m = (r + l) / 2;
        // s[m..] が tの部分文字列になっているか
        let mut j = 0;
        let tf = s[m..].iter().copied().all(|sc| {
            if let Some(jj) = t[j..].iter().copied().position(|tc| tc == sc) {
                j += jj + 1;
                true
            } else {
                false
            }
        });
        if tf {
            r = m;
        } else {
            l = m;
        }
    }
    let rs = r;
    println!("{rs}");
}
