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
        q: usize,
        cc: [Usize1; n],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut counters = vec![counter::Counter::<usize, usize>::new(); n];
    for (i, &c) in cc.iter().enumerate() {
        counters[dsu.leader(i)][&c] += 1;
    }
    for _ in 0..q {
        input! { t: usize }
        if t == 1 {
            input! { a: Usize1, b: Usize1 }
            if dsu.same(a, b) {
                continue;
            }
            let la = dsu.leader(a);
            let lb = dsu.leader(b);
            dsu.merge(a, b);
            // 大きい方にマージする
            let (sml, big) = if counters[la].len() < counters[lb].len() {
                (la, lb)
            } else {
                (lb, la)
            };
            for (k, v) in std::mem::take(&mut counters[sml]) {
                counters[big][&k] += v;
            }
            if counters[dsu.leader(a)].is_empty() {
                counters.swap(la, lb);
            }
        } else {
            input! { x: Usize1, y: Usize1 }
            let rs = counters[dsu.leader(x)][&y];
            println!("{rs}");
        }
    }
}
