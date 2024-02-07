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
        ss: [isize; n],
    };
    // s[i] = a[i] + a[i + 1] + a[i + 2]

    // a0 と a1 を適当に決めると全ての a[i] が決まる
    // a2 = s0 - a0 - a1 >= 0
    // a3 = s1 - a1 - a2 >= 0
    // a4 = s2 - a2 - a3 >= 0

    // bb[i] = [定数項, a0係数, a1係数]
    // a[i] = bb[i][0] + bb[i][1]*a0 + bb[i][2]*a1
    let mut bb = vec![[0isize; 3]; n + 2];
    bb[0][1] = 1;
    bb[1][2] = 1;
    for i in 2..(n + 2) {
        bb[i][0] = ss[i - 2];
        for j in 0..3 {
            bb[i][j] -= bb[i - 2][j];
            bb[i][j] -= bb[i - 1][j];
        }
    }
    // for (i, bb) in bb.iter().enumerate() {
    //     eprintln!("{i}: {bb:?}");
    // }
    /*
    stdin:
    5
    6 9 6 6 5
    expected:
    Yes
    0 4 2 3 1 2 2
    actual:
    0: [0, 1, 0]
    1: [0, 0, 1]
    2: [6, -1, -1]
    3: [3, 1, 0]
    4: [-3, 0, 1]
    5: [6, -1, -1]
    6: [2, 1, 0]
         */
    // i= 3m   番前は bb[i][0] +a0     >= 0
    // i= 3m+1 番前は bb[i][1]     +a1 >= 0
    // i= 3m+2 番前は bb[i][2] -a0 -a1 >= 0
    let mut kk = [isize::MAX, isize::MAX, isize::MAX];
    for (i, bb) in bb.iter().copied().enumerate() {
        match i % 3 {
            0 => {
                kk[0] = kk[0].min(bb[0]);
            }
            1 => {
                kk[1] = kk[1].min(bb[0]);
            }
            2 => {
                kk[2] = kk[2].min(bb[0]);
            }
            _ => unreachable!(),
        }
    }
    // kk[0] + a0 >= 0
    // kk[1] + a1 >= 0
    // kk[2] - a0 - a1 >= 0
    // を満たす a0, a1 が存在するか
    // -kk[0] <= a0
    // -kk[1] <= a1
    // a0 + a1 <= kk[2]
    // eprintln!("{kk:?}");
    let a0 = (-kk[0]).max(0);
    let a1 = (-kk[1]).max(0);
    if kk[2] < a0 + a1 {
        println!("No");
    } else {
        println!("Yes");
        let mut rs = vec![0; n + 2];
        rs[0] = a0;
        rs[1] = a1;
        for i in 2..(n + 2) {
            rs[i] = bb[i][0] + bb[i][1] * a0 + bb[i][2] * a1;
        }
        println!("{}", rs.iter().join(" "));
    }
}
