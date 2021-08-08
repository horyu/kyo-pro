#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
        mut cc: [usize; n],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    cc.sort_unstable();
    let mut aa: VecDeque<_> = aa.iter().collect();
    let mut bb: VecDeque<_> = bb.iter().collect();
    let mut cc: VecDeque<_> = cc.iter().collect();
    let mut cnt = 0;
    'a: while let Some(a) = aa.pop_front() {
        while let Some(b) = bb.pop_front() {
            if a < b {
                while let Some(c) = cc.pop_front() {
                    if b < c {
                        cnt += 1;
                        continue 'a;
                    }
                }
                break 'a;
            }
        }
        break;
    }
    println!("{}", cnt);
}
