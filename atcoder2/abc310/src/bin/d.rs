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
        t: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m],
    };
    let mut ngs = vec![vec![false; n]; n];
    for (a, b) in aabb {
        ngs[a][b] = true;
        ngs[b][a] = true;
    }
    let mut ttt = vec![vec![]; t];
    let rs = dfs(&ngs, n, t, 0, &mut ttt);
    println!("{rs}");
}

fn dfs(ngs: &[Vec<bool>], n: usize, t: usize, crr: usize, ttt: &mut Vec<Vec<usize>>) -> usize {
    let empty_cnt = ttt.iter().filter(|tt| tt.is_empty()).count();
    if n - crr < empty_cnt {
        return 0;
    }
    if crr == n - 1 {
        if 1 == empty_cnt {
            return 1;
        }
        return ttt
            .iter()
            .filter(|&tt| tt.iter().copied().all(|t| !ngs[t][crr]))
            .count();
    }
    let mut ans = 0;
    for ti in 0..t {
        if ttt[ti].iter().copied().all(|t| !ngs[t][crr]) {
            ttt[ti].push(crr);
            ans += dfs(ngs, n, t, crr + 1, ttt);
            ttt[ti].pop();
        }
        if ttt[ti].is_empty() {
            break;
        }
    }
    ans
}
