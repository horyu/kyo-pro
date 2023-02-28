#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    };
    let mut cc = vec![];
    for &c in chain!(&s1, &s2, &s3) {
        cc.push(c);
    }
    cc.sort_unstable();
    cc.dedup();
    let len = cc.len();
    if 10 < len {
        println!("UNSOLVABLE");
        return;
    }
    for ii in (0usize..=9).permutations(len) {
        let c_to_num = |c: char| -> usize { ii[cc.iter().position(|&cc| cc == c).unwrap()] };
        if [s1[0], s2[0], s3[0]].iter().any(|&c| c_to_num(c) == 0) {
            continue;
        }
        let s_to_num =
            |s: &[char]| -> usize { s.iter().copied().fold(0, |acc, c| acc * 10 + c_to_num(c)) };
        let n1 = s_to_num(&s1);
        let n2 = s_to_num(&s2);
        let n3 = s_to_num(&s3);
        if n1 + n2 == n3 {
            println!("{n1}\n{n2}\n{n3}");
            return;
        }
    }
    println!("UNSOLVABLE");
}
