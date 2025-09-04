#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        l: usize,
    };
    // 2のブロックについて、先頭の2を除くとj番目は値がjになる
    // 0と1のブロックは２のブロックの値をkとしたとき (k + 1) % 2 で置換したら良い
    let vv2 = (0..n)
        .map(|mut j| {
            let mut vv = vec![0; l - 1];
            let mut index = l - 2;
            while j != 0 {
                vv[index] = j % 3;
                j /= 3;
                index = index.saturating_sub(1);
            }
            vv
        })
        .collect_vec();
    for block in 0..3 {
        let vvx = vv2
            .iter()
            .map(|vv| {
                vv.iter()
                    .copied()
                    .map(|v| (v + 1 + block) % 3)
                    .collect_vec()
            })
            .collect_vec();
        for vv in vvx {
            println!("{block}{}", vv.iter().join(""));
        }
    }
}

/*
@ 2 2
00
02
11
12
20
21

@ 2 3
020
022
111
112
200
201

@ 2 4
0220
0222
1111
1112
2000
2001

@ 3 2
00
01
02
10
11
12
20
21
22

@ 3 3
010
011
012
120
121
122
200
201
202

@ 4 3
010
010
011
022
120
120
121
102
200
201
202
210
*/
