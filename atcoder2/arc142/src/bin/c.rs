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
    use std::io::{self, Write};
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut s = String::new();

    stdin.read_line(&mut s).unwrap();
    let n = s.trim_end().parse::<usize>().unwrap();

    let mut ddd = vec![vec![n; n]; 2];
    for i in 1..=2 {
        for j in 3..=n {
            println!("? {i} {j}");
            stdout.flush().unwrap();
            s.clear();
            stdin.read_line(&mut s).unwrap();
            let d = s.trim_end().parse::<usize>().unwrap();
            ddd[i - 1][j - 1] = d
        }
    }

    let ee = izip!(&ddd[0], &ddd[1])
        .map(|(di, dj)| di + dj)
        .collect_vec();
    let mut rs = ee.iter().min().copied().unwrap();
    if rs == 3 {
        let ii = ee.iter().copied().positions(|e| e == 3).collect_vec();
        if 2 <= ii.len() {
            let i = ii[0] + 1;
            let j = ii[1] + 1;
            println!("? {i} {j}");
            stdout.flush().unwrap();
            s.clear();
            stdin.read_line(&mut s).unwrap();
            let d = s.trim_end().parse::<usize>().unwrap();
            if d != 1 {
                rs = 1;
            }
        } else {
            rs = 1;
        }
    }
    println!("! {rs}");
    stdout.flush().unwrap();
}
