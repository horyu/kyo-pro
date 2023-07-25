#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        ww: [usize; n],
        kk: [Usize1; q],
    };
    let ww = ww.repeat(2);
    let all = ww[..n].iter().sum::<usize>();
    let mut count = vec![(x / all) * n; n];
    let x = x % all;

    let mut j = 0;
    let mut s = 0;
    for i in 0..n {
        if j < i {
            j = i;
            s = 0;
        }
        while s < x {
            s += ww[j];
            j += 1;
        }
        count[i] += j - i;
        s = s.saturating_sub(ww[i]);
    }

    let mut order = vec![None; n];
    let mut path = vec![];
    let mut loop_size = 0;
    let mut u = 0;
    for k in 0usize.. {
        if let Some(o) = order[u] {
            loop_size = k - o;
            break;
        }
        order[u] = Some(k);
        path.push(u);
        u = (u + count[u]) % n;
    }

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
