#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        aa: [Usize1; m],
        uuvv: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (i, (u, v)) in uuvv.iter().copied().enumerate() {
        g[u].push((v, i));
        g[v].push((u, i));
    }
    let mut cc = vec![0; n - 1];
    for (start, goal) in aa.iter().copied().tuple_windows() {
        let mut qq = VecDeque::new();
        // 直前の頂点, 辺
        let mut ff = vec![(!0, !0); n];
        qq.push_back((start, !0usize));
        while let Some((qi, qf)) = qq.pop_front() {
            for (i, j) in g[qi].iter().copied() {
                if i == qf {
                    continue;
                }
                ff[i] = (qi, j);
                if i == goal {
                    break;
                }
                qq.push_back((i, qi));
            }
        }
        let mut i = goal;
        while i != start {
            // eprintln!("{start}: {i} {:?}", ff[i]);
            cc[ff[i].1] += 1;
            i = ff[i].0;
        }
    }
    // eprintln!("{cc:?}");
    let mut hm = HashMap::new();
    hm.insert(0isize, ModInt998244353::new(1));
    for c in cc {
        let mut new_hm = HashMap::new();
        for (k, v) in hm {
            *new_hm.entry(k + c).or_default() += v;
            *new_hm.entry(k - c).or_default() += v;
        }
        hm = new_hm;
    }
    let rs = hm.get(&k).copied().unwrap_or_default();
    println!("{rs}");
}
