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
        q: usize,
        x: usize,
        ww: [usize; n],
        kk: [Usize1; q],
    };
    let ww_sum = ww.iter().sum::<usize>();

    // i番目のじゃがいもから入れ始めた箱の中身の個数
    let mut count = vec![(x / ww_sum) * n; n];
    {
        let xx = x % ww_sum;
        let ww = ww.repeat(2);
        let mut r = 0;
        let mut sum = 0;
        for l in 0..n {
            if r < l {
                r = l;
                sum = 0;
            }
            while sum < xx {
                sum += ww[r];
                r += 1;
            }
            count[l] += r - l;
            sum -= ww[l];
        }
    }

    let mut path = vec![];
    let loop_size = {
        let mut order = vec![usize::MAX; n];
        let mut i = 0;
        let mut u = 0;
        while order[u] == usize::MAX {
            order[u] = i;
            path.push(u);
            u = (u + count[u]) % n;
            i += 1;
        }
        i - order[u]
    };

    let non_loop = path.len() - loop_size;
    for k in kk {
        let rs = if k < non_loop {
            count[path[k]]
        } else {
            count[path[non_loop + (k - non_loop) % loop_size]]
        };
        println!("{rs}");
    }
}
