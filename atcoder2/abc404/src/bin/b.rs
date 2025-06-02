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
        mut ss: [Chars; n],
        tt: [Chars; n],
    };
    let mut rs = !0;
    for rotate in 0..4 {
        let diff = izip!(&ss, &tt)
            .map(|(s, t)| izip!(s, t).filter(|(s, t)| s != t).count())
            .sum::<usize>();
        rs = rs.min(rotate + diff);

        // ss を90度回転させる
        let mut new_ss = vec![vec![' '; n]; n];
        for i in 0..n {
            for j in 0..n {
                new_ss[j][n - 1 - i] = ss[i][j];
            }
        }
        ss = new_ss;
    }
    println!("{rs}");
}
