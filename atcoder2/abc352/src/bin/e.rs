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
        m: usize,
    };
    let mut cc = vec![];
    let mut aaa = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            c: usize,
            aa: [Usize1; k],
        };
        cc.push(c);
        aaa.push(aa);
    }
    let mut dsu = ac_library::Dsu::new(n);
    let mut rs = 0;
    for i in (0..m).sorted_unstable_by_key(|&i| cc[i]) {
        let c = cc[i];
        let mut news = vec![];
        let mut olds = HashSet::new();
        for a in aaa[i].iter().copied() {
            if dsu.size(a) == 1 {
                news.push(a);
            } else {
                olds.insert(dsu.leader(a));
            }
        }

        // 新規頂点軍を結合
        for (nl, nr) in news.iter().copied().tuple_windows() {
            dsu.merge(nl, nr);
            rs += c;
        }
        // 既存頂点を結合
        for (ol, or) in olds.iter().copied().tuple_windows() {
            dsu.merge(ol, or);
            rs += c;
        }
        // 新規と既存を結合
        if let Some(n) = news.first().copied() {
            if let Some(o) = olds.iter().copied().next() {
                dsu.merge(n, o);
                rs += c;
            }
        }
        // eprintln!("[{i}] {rs}: {c} {news:?} {olds:?} {:?}", aaa[i]);
    }
    if dsu.size(0) != n {
        println!("-1");
        return;
    }
    println!("{rs}");
}
