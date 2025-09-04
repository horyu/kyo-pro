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
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        ss: [Chars; h],
    };
    // 01-BFS
    let mut qq = VecDeque::new();
    let mut counter = counter::Counter::<(usize, usize)>::new();
    counter[&(ch, cw)] = 0;
    qq.push_back((ch, cw, 0));
    while let Some((qi, qj, qm)) = qq.pop_front() {
        if qi == dh && qj == dw {
            println!("{qm}");
            return;
        }
        for i in qi.saturating_sub(2)..h.min(qi + 3) {
            for j in qj.saturating_sub(2)..w.min(qj + 3) {
                let m = qm + (1 < qi.abs_diff(i) + qj.abs_diff(j)) as usize;
                if ss[i][j] == '.' && m < counter.get(&(i, j)).copied().unwrap_or(usize::MAX) {
                    counter[&(i, j)] = m;
                    if m == qm {
                        qq.push_front((i, j, m));
                    } else {
                        qq.push_back((i, j, m));
                    }
                }
            }
        }
    }
    println!("-1");
}

#[allow(dead_code)]
fn main2() {
    input! {
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        ss: [Chars; h],
    };
    use std::cmp::Reverse;
    let mut bh = BinaryHeap::new();
    let mut dd = vec![vec![!0usize; w]; h];
    bh.push((Reverse(0), ch, cw));
    dd[ch][cw] = 0;
    while let Some((Reverse(qm), qi, qj)) = bh.pop() {
        for i in qi.saturating_sub(2)..h.min(qi + 3) {
            for j in qj.saturating_sub(2)..w.min(qj + 3) {
                let m = qm + (1 < qi.abs_diff(i) + qj.abs_diff(j)) as usize;
                if ss[i][j] == '.' && m < dd[i][j] {
                    // eprintln!("{} {qm}->{m} ({qi},{qj})->({i},{j})", dd[i][j]);
                    dd[i][j] = m;
                    bh.push((Reverse(m), i, j));
                }
            }
        }
    }
    if dd[dh][dw] == !0 {
        println!("-1");
    } else {
        println!("{}", dd[dh][dw]);
    }
}
