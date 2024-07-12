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

// https://atcoder.jp/contests/abc343/submissions/52197555
use ac_library::{Monoid, Segtree};
type U4 = ((usize, usize), (usize, usize));
struct M;
impl Monoid for M {
    type S = U4;

    fn identity() -> Self::S {
        ((0, 0), (0, 0))
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let (p, q);
        if a.0 .0 == b.0 .0 {
            p = (a.0 .0, a.0 .1 + b.0 .1);
            if a.1 .0 == b.1 .0 {
                q = (a.1 .0, a.1 .1 + b.1 .1);
            } else {
                q = a.1.max(b.1);
            }
        } else if b.0 < a.0 {
            p = a.0;
            if a.1 .0 == b.0 .0 {
                q = (a.1 .0, a.1 .1 + b.0 .1);
            } else {
                q = a.1.max(b.0);
            }
        } else {
            p = b.0;
            if a.0 .0 == b.1 .0 {
                q = (a.0 .0, a.0 .1 + b.1 .1);
            } else {
                q = a.0.max(b.1);
            }
        }
        (p, q)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        ttxxyy: [(usize, Usize1, usize); q],
    };
    let mut st = Segtree::<M>::from_iter(aa.iter().copied().map(|a| ((a, 1), (0, 0))));
    for (t, x, y) in ttxxyy {
        if t == 1 {
            st.set(x, ((y, 1), (0, 0)))
        } else {
            let rs = st.prod(x..y).1 .1;
            println!("{rs}");
        }
    }
}
