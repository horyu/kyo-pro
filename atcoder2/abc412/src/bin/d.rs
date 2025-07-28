#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut g = vec![0; n];
    for (a, b) in aabb {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
    }
    // ii の順でサイクルを作る際に元からある辺の数
    let f = |ii: &[usize]| -> i32 {
        debug_assert!(ii.len() >= 3);
        let ii = ii.iter().copied().collect_vec();
        let mut checked = 0;
        let mut tmp = 0;
        for (i, j) in ii.iter().copied().cycle().tuple_windows().take(ii.len()) {
            tmp += (g[i] & (1 << j) != 0) as i32;
            checked |= 1 << i;
        }
        tmp
    };
    let nn = n as i32;
    let mm = m as i32;
    let mut rs = i32::MAX;
    for ii in (0..n).permutations(n) {
        let v = f(&ii);
        rs = rs.min(mm - v + (nn - v));
        if 6 <= n {
            for mid in 3..=(n - 3) {
                let x = f(&ii[..mid]);
                let y = f(&ii[mid..]);
                rs = rs.min(mm - x - y + (nn - x - y));
            }
        }
    }
    println!("{rs}");
}
