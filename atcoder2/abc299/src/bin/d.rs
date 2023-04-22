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
    use std::io::{self, Write};
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut s = String::new();

    stdin.read_line(&mut s).unwrap();
    let n = s.trim_end().parse::<usize>().unwrap();

    let mut zero = 0;
    let mut one = n - 1;
    while zero + 1 < one {
        let m = (zero + one) / 2;
        println!("? {}", m + 1);
        stdout.flush().unwrap();

        s.clear();
        stdin.read_line(&mut s).unwrap();
        let c = s.trim_end().parse::<usize>().unwrap();
        if c == 0 {
            zero = m;
        } else {
            one = m;
        }
    }

    println!("! {}", zero + 1);
    stdout.flush().unwrap();
}
