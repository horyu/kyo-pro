#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(get_many_mut)]
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
        q: usize,
        ttaabb: [(usize, Usize1, Usize1); q],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut iii = (0..n).map(|i| vec![i]).collect_vec();
    for (t, a, b) in ttaabb {
        if t == 1 {
            if !dsu.same(a, b) {
                let (la, lb) = (dsu.leader(a), dsu.leader(b));
                let (from, to) = if la == dsu.merge(a, b) {
                    (lb, la)
                } else {
                    (la, lb)
                };
                if let Ok([ff, tt]) = iii.get_many_mut([from, to]) {
                    tt.append(ff);
                    tt.sort_unstable_by_key(|&i| R(i));
                    tt.truncate(10);
                }
            }
            continue;
        }
        let la = dsu.leader(a);
        if let Some(v) = iii[la].get(b) {
            let rs = v + 1;
            println!("{rs}");
        } else {
            println!("-1");
        }
    }
}
