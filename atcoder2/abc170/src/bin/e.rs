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

use ac_library::{LazySegtree, MapMonoid, Min};
struct MinMonoid;
impl MapMonoid for MinMonoid {
    type M = Min<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &usize, &x: &usize) -> usize {
        f + x
    }

    fn composition(&f: &usize, &g: &usize) -> usize {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        mut aabb: [(usize, Usize1); n],
        ccdd: [(Usize1, Usize1); q],
    };
    let mut ens = vec![BTreeSet::<(usize, usize)>::new(); 2e5 as usize];
    for (i, (rate, j)) in aabb.iter().copied().enumerate() {
        ens[j].insert((rate, i));
    }
    let mut ls = LazySegtree::<MinMonoid>::from(vec![1e10 as usize; 2e5 as usize]);
    for (j, en) in ens.iter().enumerate() {
        if let Some(&(rate, _)) = en.last() {
            ls.set(j, rate);
        }
    }
    for (c, d) in ccdd {
        // 削除
        {
            let (rate, j) = aabb[c];
            ens[j].remove(&(rate, c));
            if let Some(&(r, _)) = ens[j].last() {
                ls.set(j, r);
            } else {
                ls.set(j, 1e10 as usize);
            }
        }
        // 追加
        {
            aabb[c].1 = d;
            let (rate, j) = aabb[c];
            ens[j].insert((rate, c));
            if let Some(&(r, _)) = ens[j].last() {
                ls.set(j, r);
            }
        }

        let rs = ls.all_prod();
        println!("{rs}");
    }
}
