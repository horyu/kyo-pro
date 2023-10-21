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
        s: Bytes,
        t: Bytes,
    };
    let mut cnts = [0; 1000];
    let mut sa = 0;
    let mut ta = 0;
    for sc in s {
        if sc == b'@' {
            sa += 1;
            continue;
        }
        cnts[sc as usize] += 1;
    }
    for tc in t {
        if tc == b'@' {
            ta += 1;
            continue;
        }
        cnts[tc as usize] -= 1;
    }
    for i in 'a'..='z' {
        let cnt = cnts[i as u8 as usize];
        if "atcoder".contains(i) {
            match cnt.cmp(&0) {
                std::cmp::Ordering::Less => {
                    sa -= cnt;
                    if sa < 0 {
                        println!("No");
                        return;
                    }
                }
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => {
                    ta -= cnt;
                    if ta < 0 {
                        println!("No");
                        return;
                    }
                }
            }
        } else if cnt != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
