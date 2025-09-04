#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        uuvv: [(Usize1, Usize1); m],
    };
    // https://qiita.com/u2dayo/items/3d911628cb556a40d86d#e%E5%95%8F%E9%A1%8Cjust-one
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut checked = vec![false; n];
    let mut rs = ModInt998244353::new(1);
    for i in 0..n {
        if checked[i] {
            continue;
        }
        checked[i] = true;
        let mut qq = VecDeque::new();
        qq.push_back(i);
        let mut vc = 0;
        let mut ec = 0;
        while let Some(qi) = qq.pop_front() {
            vc += 1;
            for j in g[qi].iter().copied() {
                ec += 1;
                if checked[j] {
                    continue;
                }
                checked[j] = true;
                qq.push_back(j);
            }
        }
        rs *= if vc == ec / 2 { 2 } else { 0 };
    }
    println!("{rs}");
}
