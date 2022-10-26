#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        d: usize,
        k: usize,
        llrr: [(Usize1, Usize1); d],
        sstt: [(Usize1, Usize1); k],
    };
    let mut ii = (0..k).collect_vec();
    let mut cc = sstt.iter().map(|st| st.0).collect_vec();
    let mut rrss = vec![0; k];
    for (day, (l, r)) in llrr.into_iter().enumerate() {
        for j in (0..ii.len()).rev() {
            let i = ii[j];
            let c = &mut cc[i];
            let t = sstt[i].1;
            if (l..=r).contains(c) {
                if t < l {
                    *c = l.min(*c);
                } else if r < t {
                    *c = r.max(*c);
                } else {
                    // goal
                    rrss[i] = day + 1;
                    ii.swap_remove(j);
                }
            }
        }
    }
    println!("{}", rrss.iter().join("\n"));
}
