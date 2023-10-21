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
        m: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let all_sum = aa.iter().sum::<usize>();
    let mut rs = all_sum;
    aa.extend(aa.clone().into_iter().map(|a| a + m));
    let mut i = 0;
    while i < 2 * n {
        let mut sum = 0;
        let mut pre = aa[i];
        while i < 2 * n && aa[i].abs_diff(pre) <= 1 {
            let a = aa[i];
            sum += if m <= a { a - m } else { a };
            pre = a;
            i += 1;
        }
        rs = rs.min(all_sum.saturating_sub(sum));
    }
    println!("{rs}");
}
