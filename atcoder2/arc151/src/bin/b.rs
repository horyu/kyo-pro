#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use ac_library_rs::ModInt998244353;
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::AddAssign,
};

fn main() {
    // input! {
    //     n: usize,
    //     m: usize,
    //     pp: [Usize1; n],
    // };
    // let mut rs = ModInt998244353::new(0);
    // let mut free = n as u32;
    // let mm = ModInt998244353::new(m);
    // // let mut ttff = vec![false; n];
    // let mut dsu = ac_library_rs::Dsu::new(n);
    // for i in 0..n {
    //     let p = pp[i];
    //     if !dsu.same(i, p) {
    //         if i != p {
    //             let is = dsu.size(i);
    //             let ps = dsu.size(p);
    //             eprintln!("{i}:{is} {p}:{ps} {free}");
    //             // rs += mm * (mm - 1) / 2 * m.pow(free - (is + ps) as u32);
    //             if is == 1 && ps == 1 {
    //                 rs += mm * (mm - 1) / 2 * m.pow(free - 2);
    //                 dbg!(mm * (mm - 1) / 2 * m.pow(free - 2));
    //             } else {

    //             }
    //         }
    //         dsu.merge(i, p);
    //         free -= 1;
    //     }
    //     // ttff[i] = true;
    //     // ttff[p] = true;
    // }

    // println!("{rs}");
}
