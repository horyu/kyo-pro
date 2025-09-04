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
        s: Chars,
    };
    // s[i] からみて左側にある 1 の個数
    let mut ll = vec![0; n];
    for i in 1..n {
        ll[i] = ll[i - 1] + usize::from(s[i - 1] == '1');
    }
    // s[i] からみて右側にある 1 の個数
    let mut rr = vec![0; n];
    for i in (0..(n - 1)).rev() {
        rr[i] = rr[i + 1] + usize::from(s[i + 1] == '1');
    }
    let mut rs = 0;
    for (i, c) in s.into_iter().enumerate() {
        if c == '0' {
            rs += ll[i].min(rr[i]);
        }
    }
    println!("{rs}");
}
