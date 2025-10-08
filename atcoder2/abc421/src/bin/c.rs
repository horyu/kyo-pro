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

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut rs = !0;
    // A,Bが隣り合わないようにするのに必要な回数 = min(/AB/ループにする回数, /BA/ループにする回数)
    for l in ['A', 'B'] {
        let mut x = 0;
        let mut y = 0;
        let mut vv = vec![];
        for c in s.iter().copied() {
            if c == l {
                vv.push(x * 2);
                x += 1;
            } else {
                vv.push(y * 2 + 1);
                y += 1;
            }
        }
        // eprintln!("{vv:?}");
        // BIT
        let mut tmp = 0;
        let mut ft = ac_library::FenwickTree::new(n * 2, 0usize);
        for &v in vv.iter().rev() {
            tmp += ft.sum(..=v);
            ft.add(v, 1);
        }
        rs = rs.min(tmp);
    }
    println!("{rs}");
}
