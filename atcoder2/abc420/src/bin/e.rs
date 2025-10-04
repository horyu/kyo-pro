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
        q: usize,
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut counter = counter::Counter::<_>::new();
    let mut ttff = vec![false; n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }
            let lu = dsu.leader(u);
            let lv = dsu.leader(v);
            if lu == lv {
                continue;
            }
            let uc = counter[&lu];
            let vc = counter[&lv];
            let nc = uc + vc;
            counter[&dsu.merge(u, v)] = nc;
        } else if t == 2 {
            input! {
                v: Usize1,
            }
            if ttff[v] {
                counter[&dsu.leader(v)] -= 1;
            } else {
                counter[&dsu.leader(v)] += 1;
            }
            ttff[v] ^= true;
        } else {
            input! {
                v: Usize1,
            }
            let tf = 0 < counter[&dsu.leader(v)];
            let rs = ["No", "Yes"][tf as usize];
            println!("{rs}");
        }
    }
}
