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
    let mut hs = HashSet::new();
    for &c in chain!(&s1, &s2, &s3) {
        hs.insert(c);
    }
    let len = hs.len();
    if 10 < len {
        println!("UNSOLVABLE");
        return;
    }
    let cc = hs.into_iter().collect_vec();
    let s_to_num = |s: &[char], ii: &[usize]| -> usize {
        s.iter().fold(0, |acc, &c| {
            acc * 10 + ii[cc.iter().position(|&cc_c| cc_c == c).unwrap()]
        })
    };
    // eprintln!("{}", cc.iter().join(""));
    for ii in (0usize..=9).permutations(len) {
        let n1 = s_to_num(&s1, &ii);
        let n2 = s_to_num(&s2, &ii);
        let n3 = s_to_num(&s3, &ii);
        // eprintln!("{n1} {n2} {n3} : {}", ii.iter().join(","));
        if [(n1, &s1), (n2, &s2), (n3, &s3)]
            .into_iter()
            .any(|(n, s)| n == 0 || n.ilog10() + 1 != s.len() as u32)
        {
            continue;
        }
        if n1 + n2 == n3 {
            println!("{n1}\n{n2}\n{n3}");
            return;
        }
    }
    println!("UNSOLVABLE");
}
