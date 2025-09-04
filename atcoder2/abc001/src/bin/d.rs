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
        ss: [String; n],
    };
    // 1日を00:00から24:00まで5分刻みで区切る
    let mut ttff = vec![false; 60 / 5 * 24 + 1];
    for s in ss {
        //     012345678
        // s: "hhmm-hhmm"
        let lh = s[0..2].parse::<usize>().unwrap();
        let lm = s[2..4].parse::<usize>().unwrap() / 5;

        let mut rh = s[5..7].parse::<usize>().unwrap();
        let mut rm = s[7..9].parse::<usize>().unwrap().div_ceil(5);
        if rm == 60 {
            rh += 1;
            rm = 0;
        }
        for i in (lh * 12 + lm)..(rh * 12 + rm) {
            ttff[i] = true;
        }
    }
    // eprintln!("{}", ttff.iter().map(|tf| if *tf { 'T' } else { 'F' }).collect::<String>());
    let mut pre = false;
    ttff.push(false);
    for (i, tf) in ttff.into_iter().enumerate() {
        if !pre && tf {
            print!("{:02}{:02}-", i / 12, i % 12 * 5);
        } else if pre && !tf {
            println!("{:02}{:02}", i / 12, i % 12 * 5);
        }
        pre = tf;
    }
}
