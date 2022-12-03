#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library_rs::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::AddAssign,
};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
    };
    if w == 1 {
        println!("1");
        return;
    }

    let g = (1..w)
        .map(|_| [false, true])
        .multi_cartesian_product()
        .filter(|ttff| ttff.iter().tuple_windows().all(|(x, y)| !x || !y))
        .collect_vec();
    // 左下、真下、右下　への経路数
    let mut mm = vec![(0, 0, 0); w];
    for ttff in g {
        // eprintln!("{:?}", ttff);
        if ttff[0] {
            mm[0].2 += 1;
        } else {
            mm[0].1 += 1;
        }
        for i in 1..(w - 1) {
            match (ttff[i - 1], ttff[i]) {
                (true, true) => unreachable!(),
                (true, false) => mm[i].0 += 1,
                (false, true) => mm[i].2 += 1,
                (false, false) => mm[i].1 += 1,
            }
        }
        if ttff[w - 2] {
            mm[w - 1].0 += 1;
        } else {
            mm[w - 1].1 += 1;
        }
    }
    let mut vv = vec![ModInt1000000007::new(0); w];
    vv[0] += 1;
    for _ in 0..h {
        let mut new_vv = vec![ModInt1000000007::new(0); w];
        for (i, (l, m, r)) in mm.iter().copied().enumerate() {
            if 0 < i {
                new_vv[i - 1].add_assign(vv[i] * l);
            }
            new_vv[i].add_assign(vv[i] * m);
            if i < w - 1 {
                new_vv[i + 1].add_assign(vv[i] * r);
            }
        }
        vv = new_vv;
    }
    // eprintln!("{:?}", mm);
    // eprintln!("{:?}", vv);
    println!("{}", vv[k - 1]);
}
