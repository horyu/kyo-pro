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
