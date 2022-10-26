#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        t: usize,
        nnkkss: [(usize, usize, Chars); t],
    };
    for (n, k, s) in nnkkss {
        let mut ones = vec![0];
        let mut zeros = vec![0];
        let mut qs = vec![0];
        for &c in &s {
            ones.push(*ones.last().unwrap());
            zeros.push(*zeros.last().unwrap());
            qs.push(*qs.last().unwrap());
            match c {
                '0' => *zeros.last_mut().unwrap() += 1,
                '1' => *ones.last_mut().unwrap() += 1,
                _ => *qs.last_mut().unwrap() += 1,
            }
        }
        if k < ones[n] || ones[n] + qs[n] < k {
            println!("No");
            continue;
        }
        let mut kouho = 0;
        for i in 0..=(n - k) {
            if zeros[i + k] != zeros[i] {
                continue;
            }
            if 0 < i && s[i - 1] == '1' {
                continue;
            }
            if i < n - k && s[i + k] == '1' {
                continue;
            }
            if ones[n] + qs[i + k] - qs[i] == k {
                kouho += 1;
            }
        }
        if kouho == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
