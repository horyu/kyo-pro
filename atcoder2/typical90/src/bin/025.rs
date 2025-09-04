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
        b: usize,
    };
    let mut rs = 0;
    for i in 1..=11 {
        rs += dfs(i, 9, &Vec::new(), n, b);
    }
    if i_to_vec(b).contains(&0) && b <= n {
        rs += 1;
    }

    println!("{rs}");
}

fn i_to_vec(i: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut tmp = i;
    while tmp != 0 {
        v.push(tmp % 10);
        tmp /= 10;
    }
    v
}

fn dfs(pos: usize, last: usize, vv: &Vec<usize>, n: usize, b: usize) -> usize {
    let mut cnt = 0;
    if pos == 0 {
        let mut rem = 1;
        for &v in vv {
            rem *= v;
        }
        let goal = rem + b;
        if goal <= n {
            let mut ww = i_to_vec(goal);
            ww.sort_unstable();
            ww.reverse();
            if ww == *vv {
                cnt += 1;
            }
        }
        return cnt;
    }

    for i in 1..=last {
        let mut ww = vv.clone();
        ww.push(i);
        cnt += dfs(pos - 1, i, &ww, n, b);
    }

    cnt
}
