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

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            pp: [usize; 1 << n],
        };
        let len = pp.len();
        use ac_library::{Min, Segtree};
        // 再帰的に中心で分割→要素で見て辞書順早い方を含む方を左に配置
        let st: Segtree<Min<usize>> = Segtree::<Min<usize>>::from_iter(pp.iter().copied());
        let mut rrss = Vec::with_capacity(len);
        fn f(
            l: usize,
            r: usize,
            st: &Segtree<Min<usize>>,
            pp: &[usize],
            rrss: &mut Vec<usize>,
        ) {
            if l == r {
                rrss.push(pp[l]);
                return;
            }
            let m = (l + r).div_ceil(2);
            let left = st.prod(l..m);
            let right = st.prod(m..=r);
            if left < right {
                f(l, m - 1, st, pp, rrss);
                f(m, r, st, pp, rrss);
            } else {
                f(m, r, st, pp, rrss);
                f(l, m - 1, st, pp, rrss);
            }
        }
        f( 0, len - 1, &st, &pp, &mut rrss);
        let rs = rrss.into_iter().join(" ");
        println!("{rs}");
    }
}
