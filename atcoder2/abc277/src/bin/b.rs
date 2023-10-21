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
        ss: [Chars; n],
    };
    let mut tf = true;
    tf &= ss.iter().all(|s| matches!(s[0], 'H' | 'D' | 'C' | 'S'));
    // dbg!(tf);
    tf &= ss.iter().all(|s| {
        matches!(
            s[1],
            'A' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K'
        )
    });
    // dbg!(tf);
    tf &= ss.into_iter().sorted().dedup().count() == n;
    // dbg!(tf);

    let rs = ["No", "Yes"][tf as usize];
    println!("{rs}");
}
