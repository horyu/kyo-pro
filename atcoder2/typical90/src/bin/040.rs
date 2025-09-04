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
        w: isize,
        aa: [isize; n],
    };
    // https://qiita.com/tanaka-a/items/fb8d84c44190c7098047#%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B31%E5%85%AC%E5%BC%8F%E8%A7%A3%E8%AA%AC%E6%96%B9%E5%BC%8F
    let mut g = ac_library::maxflow::MfGraph::new(n + 2);
    let mut sum = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        g.add_edge(0, i + 1, a);
        g.add_edge(i + 1, n + 1, w);
        sum += a;
    }
    for i in 0..n {
        input! {
            k: usize,
            cc: [Usize1; k],
        }
        for c in cc {
            g.add_edge(c + 1, i + 1, 1e15 as isize);
        }
    }
    let rs = sum - g.flow(0, n + 1);
    println!("{rs}");
}
