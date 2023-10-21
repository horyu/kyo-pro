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

use ac_library::{Additive, LazySegtree, MapMonoid};
struct AddMonoid;
impl MapMonoid for AddMonoid {
    type M = Additive<usize>;
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
        aa: [usize; n],
        q: usize,
    };
    let mut hm: HashMap<usize, usize> = aa.into_iter().enumerate().collect();
    let mut base = 0;
    for _ in 0..q {
        input! {t: usize};
        match t {
            1 => {
                input! {x: usize};
                base = x;
                hm.clear();
            }
            2 => {
                input! {i: Usize1, x: usize};
                *hm.entry(i).or_insert(0) += x;
            }
            _ => {
                input! {i: Usize1};
                println!("{}", hm.get(&i).unwrap_or(&0) + base);
            }
        }
    }
}
