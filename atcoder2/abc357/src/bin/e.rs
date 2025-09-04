#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        n: usize,
        i2i: [Usize1; n],
    };
    // 末端がループで、それ以外は最終的にループに繋がる有向グラフ
    let mut dsu = ac_library::Dsu::new(n);
    for (i, j) in i2i.iter().copied().enumerate() {
        dsu.merge(i, j);
    }
    let mut rs = 0;
    let mut cc = vec![0; n];
    for gg in dsu.groups() {
        // eprintln!("!{gg:?}");
        let iii =
            pathfinding::directed::strongly_connected_components::strongly_connected_components(
                &gg,
                |&i| std::iter::once(i2i[i]),
            );
        for ii in iii {
            // eprintln!("{ii:?}");
            let i = ii[0];
            let j = i2i[i];
            if cc[j] == 0 {
                // 末端
                let len = ii.len();
                rs += len.pow(2);
                for i in ii {
                    cc[i] = len;
                }
                continue;
            }
            cc[i] += 1 + cc[j];
            rs += cc[i];
        }
    }
    // eprintln!("{cc:?}");
    println!("{rs}");
}
