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
    };
    let mut mm = (1..=9).map(|i| i % m).collect_vec();
    let cc = ('1'..='9').collect_vec();
    let mut len = 0;
    let mut c = '0';
    for i in 1..=n {
        for j in (0..9).rev() {
            let jj = j + 1;
            if mm[j] == 0 {
                let new_len = i * (n / i);
                if new_len <= n && (len < new_len || (len == new_len && c < cc[j])) {
                    len = new_len;
                    c = cc[j];
                }
            }
            mm[j] = (mm[j] * 10 + jj) % m;
        }
    }
    if len == 0 {
        println!("-1");
    } else {
        println!("{}", c.to_string().repeat(len));
    }
}
