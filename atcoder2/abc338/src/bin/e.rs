#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
#![feature(iter_repeat_n)]
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
        aabb: [(Usize1, Usize1); n],
    };
    let mut rrii = vec![(false, 0); 2 * n];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        let min = a.min(b);
        let max = a.max(b);
        rrii[min] = (false, i);
        rrii[max] = (true, i);
    }
    let mut vv = vec![];
    for (i, (tf, idx)) in rrii.into_iter().enumerate() {
        if tf {
            if let Some(last) = vv.pop() {
                if last != idx {
                    println!("Yes");
                    return;
                }
            }
        } else {
            vv.push(idx);
        }
    }
    println!("No");
}

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

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n],
    };
    let mut ls = ac_library::LazySegtree::<MaxMonoid>::new(2 * n);
    for (i, (a, b)) in aabb.into_iter().enumerate() {
        if ls.get(a) != ls.get(b) {
            println!("Yes");
            return;
        }
        let min = a.min(b);
        let max = a.max(b);
        ls.apply_range(min..max, i + 1);
    }
    println!("No");
}
