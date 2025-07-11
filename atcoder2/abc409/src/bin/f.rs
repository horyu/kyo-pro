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

fn main() {
    input! {
        mut n: usize,
        q: usize,
        xxyy: [(isize, isize); n],
    };
    let mut dsu = ac_library::Dsu::new(n + q);
    let mut xxyy = xxyy.into_iter().map(|(x, y)| (x + y, x - y)).collect_vec();
    for _ in 0..n {
        input! {t: usize};
        match t {
            1 => {
                input! {a: isize, b: isize};
                let (x, y) = (a + b, a - b);
                xxyy.push((x, y));
                n += 1;
            }
            2 => {
                // TODO
            }
            _ => {
                input! {u: Usize1, v: Usize1};
                let tf = dsu.same(u, v);
                let rs = ["No", "Yes"][tf as usize];
                println!("{rs}");
            }
        }
    }
}
