#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

pub struct MonoidBitXor<S>(
    std::convert::Infallible,
    std::marker::PhantomData<fn() -> S>,
);
impl<S> Monoid for MonoidBitXor<S> {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

struct MapMonoidXor;
impl MapMonoid for MapMonoidXor {
    type M = MonoidBitXor<usize>;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(
        f: &Self::F,
        x: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        f ^ x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        ttxxyy: [(u8, usize, usize); q],
    };
    let mut ls: LazySegtree<MapMonoidXor> = aa.into();
    for (t, x, y) in ttxxyy {
        if t == 1 {
            ls.apply(x - 1, y);
        } else {
            let rs = ls.prod((x - 1)..y);
            println!("{rs}");
        }
    }
}
