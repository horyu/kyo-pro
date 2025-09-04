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
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input!(n: usize);
    let m = n.next_power_of_two().ilog2();
    println!("{m}");
    // 1..=n 内の特定の値をm回のクエリで特定する
    for k in 0..m {
        let mut q = vec![];
        for i in 1..=n {
            if (i >> k) & 1 == 1 {
                q.push(i);
            }
        }
        println!("{} {}", q.len(), q.iter().join(" "));
    }

    input!(s: Bytes);
    let rs = s.into_iter().rfold(0usize, |acc, x| acc * 2 + (x - b'0') as usize);
    if rs == 0 {
        println!("{}", 1 << m)
    } else {
        println!("{rs}");
    }

    //  1: 0001
    //  2: 0010
    //  3: 0011
    //  4: 0100
    //  5: 0101
    //  6: 0110
    //  7: 0111
    //  8: 1000
    //  9: 1001
    // 10: 1010
    // 11: 1011
    // 12: 1100
    // 13: 1101
    // 14: 1110
    // 15: 1111
}
