#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        c: u32,
        ttaa: [(u32, u32); n],
    };
    let mut vv = vec![(3, 0); 30];
    let mut arr = [0; 30];
    let mut x = c;
    for (t, a) in ttaa {
        for i in 0..30 {
            let mut xi = (x >> i) & 1u32;
            let ai = (a >> i) & 1;
            match (t, ai) {
                (1, 0) | (2, 1) => {
                    vv[i] = (t, ai);
                }
                (3, ai) => {
                    let (tj, aj) = &mut vv[i];
                    match *tj {
                        1 => {
                            if ai == 1 && *aj == 0 {
                                *tj = 2;
                                *aj = 1;
                            }
                        }
                        2 => {
                            if ai == 1 && *aj == 1 {
                                *tj = 1;
                                *aj = 0;
                            }
                        }
                        _ => {
                            *aj ^= ai;
                        }
                    }
                }
                _ => (),
            }
            let (vt, va) = vv[i];
            xi = match vt {
                1 => xi & va,
                2 => xi | va,
                _ => xi ^ va,
            };
            arr[i as usize] = xi;
        }
        // eprintln!("{}", arr.iter().join(""));
        x = arr.iter().rev().fold(0, |acc, tmp| (acc << 1) + tmp);
        println!("{}", x);
    }
}
