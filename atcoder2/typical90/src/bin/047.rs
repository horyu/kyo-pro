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
        t: Chars,
    };
    let m = 699999953;
    let to_i = |c: char| match c {
        'R' => 0,
        'G' => 1,
        'B' => 2,
        _ => unreachable!(),
    };
    let seq1 = s.into_iter().map(to_i).collect_vec();
    let seq3 = t.into_iter().map(to_i).collect_vec();
    let mut rs = 0;
    for i in 0..3 {
        let mut seq2 = vec![0; n];
        for j in 0..n {
            seq2[j] = (i - seq3[j] + 3) % 3;
        }
        let mut power3 = 1;
        let mut hash1 = 0;
        let mut hash2 = 0;
        for j in 0..n {
            hash1 = (hash1 * 3 + seq1[j]) % m;
            hash2 = (hash2 + power3 * seq2[n - j - 1]) % m;
            if hash1 == hash2 {
                rs += 1;
            }
            power3 = power3 * 3 % m;
        }
        let mut power3 = 1;
        let mut hash1 = 0;
        let mut hash2 = 0;
        for j in 0..(n - 1) {
            hash1 = (hash1 * 3 + seq1[n - j - 1]) % m;
            hash2 = (hash2 + power3 * seq2[j]) % m;
            if hash1 == hash2 {
                rs += 1;
            }
            power3 = power3 * 3 % m;
        }
    }
    println!("{rs}");
}
