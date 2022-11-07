#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::Write,
};

fn main() {
    use std::io;
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut s = String::new();

    stdin.read_line(&mut s).unwrap();
    let n = s.trim_end().parse::<usize>().unwrap();

    let mut rs = 0usize;
    let mut node = 0;
    for i in 2..=n {
        println!("? 1 {i}");
        stdout.flush().unwrap();
        s.clear();
        stdin.read_line(&mut s).unwrap();
        let distance = s.trim_end().parse::<usize>().unwrap();
        if rs < distance {
            rs = distance;
            node = i;
        }
    }
    for j in 1..=n {
        if j == node {
            continue;
        }
        println!("? {node} {j}");
        stdout.flush().unwrap();
        s.clear();
        stdin.read_line(&mut s).unwrap();
        let distance = s.trim_end().parse::<usize>().unwrap();
        if rs < distance {
            rs = distance;
        }
    }
    println!("! {rs}");
    stdout.flush().unwrap();
}
