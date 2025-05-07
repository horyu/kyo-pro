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
    // 完全なループは内部を１回別の文字にする必要がある
    // 外から流れ込むループの場合、一度外の文字に置き換えてから一緒に置換すれば余分な手が不要となる
    let mut rs = 0;
    let mut uf = UnionFind::new(26);
    for (k, v) in hm {
        if k != v {
            rs += 1;
            if !uf.union(k, v) {
                // ループしていたら別の文字に一度置き換える
                rs += 1;
            }
        }
    }
    println!("{rs}");
}
