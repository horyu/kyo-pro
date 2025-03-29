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

use ac_library::{LazySegtree, MapMonoid, Max};
struct MaxMonoid;
impl MapMonoid for MaxMonoid {
    type M = Max<usize>;
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
        aa: [usize; n],
        rrxx: [(Usize1, usize); q],
    };
    let num2pos = chain(aa.iter().copied(), rrxx.iter().map(|rx| rx.1))
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<HashMap<_, _>>();
    let mut r2xxjj = vec![vec![]; n];
    for (j, (r, x)) in rrxx.into_iter().enumerate() {
        r2xxjj[r].push((x, j));
    }
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::new(num2pos.len());
    let mut rrss = vec![1; q];
    for ((i, a), xxjj) in izip!(aa.iter().copied().enumerate(), r2xxjj) {
        let a = num2pos[&a];
        let val = ls.prod(..a);
        let aval = ls.get(a);
        ls.set(a, aval.max(val + 1));
        for (x, j) in xxjj {
            let x = num2pos[&x];
            rrss[j] = ls.prod(..=x);
        }
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}
