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
    type M = Min<isize>;
    type F = isize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &isize, &x: &isize) -> isize {
        f + x
    }

    fn composition(&f: &isize, &g: &isize) -> isize {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        sstt: [(Usize1, Usize1); m],
    };
    let mut ls = ac_library::LazySegtree::<MinMonoid>::from(vec![0; n]);
    for (s, t) in sstt.iter().copied() {
        ls.apply_range(s..=t, 1);
    }
    let mut rrss = vec![];
    for (i, (s, t)) in sstt.iter().copied().enumerate() {
        ls.apply_range(s..=t, -1);
        if ls.prod(0..n) != 0 {
            rrss.push(i + 1);
        }
        ls.apply_range(s..=t, 1);
    }
    println!("{}", rrss.len());
    for rs in rrss {
        println!("{}", rs);
    }
}
