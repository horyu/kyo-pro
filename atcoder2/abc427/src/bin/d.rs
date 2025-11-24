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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            k: Usize1,
            s: Chars,
            uuvv: [(Usize1, Usize1); m],
        };
        let ttff = s.into_iter().map(|c| c == 'A').collect_vec();
        let mut g = vec![vec![]; n];
        for (u, v) in uuvv.iter().copied() {
            g[u].push(v);
        }
        let mut memo = vec![vec![None::<bool>; k + 1]; n];
        fn f(
            pos: usize,
            k: usize,
            ttff: &[bool],
            g: &Vec<Vec<usize>>,
            memo: &mut Vec<Vec<Option<bool>>>,
        ) -> bool {
            if let Some(v) = memo[pos][k] {
                return v;
            }
            let ans = if k == 0 {
                return g[pos]
                    .iter()
                    .copied()
                    .any(|i| g[i].iter().copied().all(|j| ttff[j]));
            } else {
                g[pos]
                    .iter()
                    .copied()
                    .any(|i| g[i].iter().copied().all(|j| f(j, k - 1, ttff, g, memo)))
            };
            memo[pos][k] = Some(ans);
            ans
        }
        let tf = f(0, k, &ttff, &g, &mut memo);
        let rs = ["Bob", "Alice"][tf as usize];
        println!("{rs}");
    }
}
