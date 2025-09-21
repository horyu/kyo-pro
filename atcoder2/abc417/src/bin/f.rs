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

// https://atcoder.jp/contests/abc417/submissions/69228878
type Mint = ac_library::ModInt998244353;

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    sum: Mint,
    len: usize,
}

use ac_library::{MapMonoid, Monoid};
struct M;
impl Monoid for M {
    type S = Node;

    fn identity() -> Self::S {
        Self::S::default()
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Self::S {
            sum: a.sum + b.sum,
            len: a.len + b.len,
        }
    }
}

struct F;
impl MapMonoid for F {
    type M = M;
    type F = Option<Mint>;

    fn identity_map() -> Self::F {
        None
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if let Some(v) = f {
            Node {
                sum: *v * x.len,
                len: x.len,
            }
        } else {
            *x
        }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        llrr: [(Usize1, Usize1); m],
    };
    let mut ls = ac_library::LazySegtree::<F>::new(n);
    for (i, a) in aa.into_iter().enumerate() {
        ls.set(
            i,
            Node {
                sum: Mint::new(a),
                len: 1,
            },
        );
    }
    for (l, r) in llrr {
        let n = ls.prod(l..=r);
        let v = n.sum / Mint::new(r - l + 1);
        ls.apply_range(l..=r, Some(v));
    }
    let rs = (0..n).map(|i| ls.get(i).sum).join(" ");
    println!("{rs}");
}
