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

// https://betrue12.hateblo.jp/entry/2020/09/23/005940#%E5%8C%BA%E9%96%93%E5%A4%89%E6%9B%B4%E5%8C%BA%E9%96%93%E6%9C%80%E5%B0%8F%E5%80%A4%E5%8F%96%E5%BE%97
use ac_library::{LazySegtree, MapMonoid, Min};
struct MinMonoid;
impl MapMonoid for MinMonoid {
    type M = Min<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        !0
    }

    fn mapping(&f: &Self::F, &x: &Self::F) -> Self::F {
        if f == !0 {
            x
        } else {
            f
        }
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        if f == !0 {
            g
        } else {
            f
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        rrccll: [(Usize1, Usize1, usize); n],
    };
    let mut ls = ac_library::LazySegtree::<MinMonoid>::from(vec![h + 1; w]);
    let mut rrss = vec![0; n];
    for (i, (r, c, l)) in rrccll
        .into_iter()
        .enumerate()
        .sorted_unstable_by_key(|ix| R(ix.1 .0))
    {
        let rs = ls.prod(c..(c + l)) - 1;
        rrss[i] = rs;
        ls.apply_range(c..(c + l), rs);
        // eprintln!("[{i}] {r} {c} {l} @{rs}");
        // eprintln!("{}", (0..w).map(|j| ls.get(j)).join(""));
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
