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
        s: Bytes,
        t: Bytes,
    };
    // https://atcoder.jp/contests/abc399/editorial/12564
    let s = s.into_iter().map(|b| (b - b'a') as usize).collect_vec();
    let t = t.into_iter().map(|b| (b - b'a') as usize).collect_vec();
    if t.iter().copied().unique().count() == 26 {
        // すべての文字が使用されていたら同一でなければならない
        println!("{}", -i32::from(s != t));
        return;
    }
    let mut hm = HashMap::new();
    for (&a, &b) in izip!(&s, &t) {
        if let Some(pre) = hm.insert(a, b) {
            if pre != b {
                // 一つの文字が複数の候補になることはない
                println!("-1");
                return;
            }
        }
    }
    let mut rs = 0;
    let mut in_deg = [0; 26];
    let mut dsu = ac_library::Dsu::new(26);
    for (k, v) in hm {
        if k != v {
            rs += 1;
            in_deg[v] += 1;
            dsu.merge(k, v);
        }
    }
    // 完全なループは内部を１回別の文字にする必要がある
    // 外から接続されているループはループ内の文字を一度外の文字に変換することでループを解消できる
    for g in dsu.groups() {
        if 1 < g.len() && g.iter().all(|&x| in_deg[x] == 1) {
            rs += 1;
        }
    }
    println!("{rs}");
}
