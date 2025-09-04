#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt1000000007;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::AddAssign,
};

#[allow(clippy::assign_op_pattern)]
fn main() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
        m: usize,
        xxyy: [(Usize1, Usize1); m],
    };
    let mut ww = vec![vec![]; n];
    for (x, y) in xxyy {
        ww[x].push(y);
        ww[y].push(x);
    }
    let mut viewed = vec![false; n];
    let mut hs = HashSet::new();
    hs.insert(a);
    let mut vv = vec![ModInt1000000007::new(0); n];
    vv[a] += 1;
    for _ in 0..200 {
        let mut new_hs = HashSet::new();
        for &from in &hs {
            for &to in &ww[from] {
                if !viewed[to] && !hs.contains(&to) {
                    // eprintln!("{from}->{to}: {}+{}", vv[to], vv[from]);
                    vv[to] = vv[to] + vv[from];
                    new_hs.insert(to);
                }
            }
        }
        if new_hs.contains(&b) {
            break;
        }
        for from in hs {
            viewed[from] = true;
        }
        hs = new_hs;
    }
    let rs = vv[b];
    println!("{rs}");
}
