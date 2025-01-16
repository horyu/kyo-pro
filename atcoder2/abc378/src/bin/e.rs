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

/*
M(x,y) = (Ax + .. + Ay) % M
S(i) = M(1) + .. + M(i)

S(i) = S(i-1) + i * Ai - M * cnt(m - ai以上)

*/

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
    };

    let mut rs = 0usize;
    let mut ft = ac_library::FenwickTree::new(m, 0usize);
    let mut pre = 0;
    let mut a_sum = 0;
    for (i, a) in aa.into_iter().enumerate() {
        let a = a % m;
        // dbg!(a);
        // a_sum を足して解釈したとき ft内の値が m - a 以上のものの個数
        // [0, m) -> [a_sum, m + a_sum)
        // [m - a, m) -> [m - a + a_sum, m + a_sum)
        let cnt = if a_sum <= a {
            // dbg!(m - a + a_sum, a_sum);
            ft.sum((m - a + a_sum)..) + ft.sum(..a_sum)
        } else {
            // dbg!((m - a + a_sum) % m , a_sum);
            ft.sum(((m - a + a_sum) % m )..a_sum)
        };
        pre = pre + (i + 1) * a - m * cnt;
        rs += pre;
        ft.add(m - a_sum, 1);
        a_sum = (a_sum + a) % m;
    }
    println!("{rs}");
}
