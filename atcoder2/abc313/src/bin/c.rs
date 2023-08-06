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
        n: usize,
        mut aa: [usize; n],
    };
    aa.sort_unstable();
    let sum = aa.iter().copied().sum::<usize>();
    let bb = chain!(vec![sum / n; n - sum % n], vec![sum / n + 1; sum % n],).collect_vec();
    let rs = izip!(aa, bb).fold(0, |acc, (a, b)| acc + a.abs_diff(b)) / 2;
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        mut aa: [isize; n],
    };
    if n == 1 {
        println!("0");
        return;
    }
    aa.sort_unstable();
    // eprintln!("{:?}", aa);
    let avg = aa.iter().sum::<isize>() as f64 / n as f64;
    let floor = avg.floor() as isize;
    let ceil = avg.ceil() as isize;
    // dbg!(avg);
    let rs = aa
        .iter()
        .copied()
        .filter(|&a| a < floor)
        .fold(0, |acc, a| acc + floor - a)
        .max(
            aa.iter()
                .copied()
                .filter(|&a| ceil < a)
                .fold(0, |acc, a| acc + a - ceil),
        );
    println!("{rs}");
}
