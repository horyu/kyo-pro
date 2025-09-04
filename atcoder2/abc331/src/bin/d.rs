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
        q: usize,
        ppp: [Chars; n],
        aabbccdd: [(usize, usize, usize, usize); q],
    };
    // 平面累積和
    let mut sum = vec![vec![0; n + 1]; n + 1];
    for (i, pp) in ppp.iter().enumerate() {
        for (j, &p) in pp.iter().enumerate() {
            sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + usize::from(p == 'B');
        }
    }
    // for s in &sum {
    //     eprintln!("{}", s.iter().join(" "));
    // }
    let f = |x: usize, y: usize| -> usize {
        // x = sn + p
        // y = tn + q
        //   tn+q
        // s 11 2
        // n 11 2
        // + 11 2
        // p 33 4
        let (s, p) = x.div_mod_floor(&n);
        let (t, q) = y.div_mod_floor(&n);
        // let rs = (s * t) * sum[n][n]
        // + s * sum[n][q]
        // + t * sum[p][n]
        // + sum[p][q];
        // eprintln!("({x}, {y}) s={} p={} t={} q={} [{rs}]", s, p, t, q);
        (s * t) * sum[n][n] + s * sum[n][q] + t * sum[p][n] + sum[p][q]
    };
    // for vv in (0..=10).combinations_with_replacement(4) {
    //     let mut actual = 0;
    //     let (a, b, c, d) = (vv[0], vv[1], vv[2], vv[3]);
    //     for i in a..=c {
    //         for j in b..=d {
    //             actual += usize::from(ppp[i % n][j % n] == 'B');
    //         }
    //     }
    //     let expected = f(c + 1, d + 1) + f(a, b) - f(a, d + 1) - f(c + 1, b);
    //     assert_eq!(actual, expected, "a={} b={} c={} d={}", a, b, c, d);
    // }

    for (a, b, c, d) in aabbccdd {
        // let a = a.saturating_sub(1);
        // let b = b.saturating_sub(1);
        let c = c + 1;
        let d = d + 1;
        let mut rs = 0;
        rs += f(c, d);
        rs += f(a, b);
        rs -= f(a, d);
        rs -= f(c, b);
        println!("{rs}");
    }
}
