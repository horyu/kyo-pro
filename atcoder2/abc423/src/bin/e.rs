#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

// https://atcoder.jp/contests/abc423/editorial/13869
use ac_library::Monoid;

#[derive(Clone, Debug)]
struct S {
    val: usize,
    len: usize,
    res: usize,
}
impl ac_library::Monoid for S {
    type S = S;

    fn identity() -> Self {
        S {
            val: 0,
            len: 0,
            res: 0,
        }
    }

    fn binary_operation(l: &Self::S, r: &Self::S) -> Self::S {
        S {
            val: l.val + r.val,
            len: l.len + r.len,
            res: l.res + r.res + l.len * r.val - l.val * r.len,
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [usize; n],
        llrr: [(usize, usize); q],
    };
    let mut ss = vec![S::identity(); n + 1];
    let mut cur = S {
        val: 0,
        len: 1,
        res: 0,
    };
    ss[0] = cur.clone();
    for i in 1..=n {
        cur.val += aa[i - 1];
        ss[i] = cur.clone();
    }
    let st = ac_library::Segtree::<S>::from(ss);
    for (l, r) in llrr {
        let rs = st.prod((l - 1)..=r).res;
        println!("{rs}");
    }
}
