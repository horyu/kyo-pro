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
        n: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let mut ft = ac_library::FenwickTree::new(1e6 as usize + 1, 0isize);
    let mut bts = BTreeSet::new();
    for a in aa {
        bts.insert(a);
        ft.add(a, 1);
    }

    let mut rs = 0;
    while let Some(a) = bts.pop_first() {
        let a_cnt = ft.sum(a..=a);
        ft.add(a, -a_cnt);
        // eprintln!("{a} {a_cnt}");

        rs += a_cnt.saturating_sub(1) * a_cnt / 2;
        for k in 1..=(1e6 as usize / a) {
            let l = a * k;
            let r = (1e6 as usize + 1).min(a * (k + 1));
            let cnt = ft.sum(l..r);
            rs += (k as isize) * a_cnt * cnt;
            // if 0 < cnt {
            //     eprintln!("{k}({l}..{r}) {cnt}: {}", (k as isize) * a_cnt * cnt);
            // }
        }
    }
    println!("{rs}");
}
