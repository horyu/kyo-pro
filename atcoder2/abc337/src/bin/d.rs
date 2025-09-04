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
        k: usize,
        ss: [Chars; h],
    };
    // o.x => 012
    let ss = ss
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|c| match c {
                    'o' => 0,
                    '.' => 1,
                    'x' => 2,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut rs = !0usize;
    // 横方向
    for i in 0..h {
        if w < k {
            break;
        }
        let mut arr = [0; 3];
        let mut r = 0;
        for l in 0..(w - k + 1) {
            while r < l + k {
                arr[ss[i][r]] += 1;
                r += 1;
            }
            if arr[2] == 0 {
                rs = rs.min(arr[1]);
            }
            arr[ss[i][l]] -= 1;
        }
    }
    // // 縦方向
    for j in 0..w {
        if h < k {
            break;
        }
        let mut arr = [0; 3];
        let mut r = 0;
        for l in 0..(h - k + 1) {
            while r < l + k {
                arr[ss[r][j]] += 1;
                r += 1;
            }
            if arr[2] == 0 {
                rs = rs.min(arr[1]);
            }
            arr[ss[l][j]] -= 1;
        }
    }

    if rs == !0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
