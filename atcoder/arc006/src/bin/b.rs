#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    let stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let l = iter.next().unwrap().parse::<usize>().unwrap();
    let mut ccc = vec![];
    for _ in 0..(l + 1) {
        s.clear();
        stdin.read_line(&mut s).unwrap();
        ccc.push(s.chars().into_iter().collect_vec());
    }
    let mut j = ccc.pop().unwrap().iter().position(|&c| c == 'o').unwrap();
    while let Some(cc) = ccc.pop() {
        if j != 0 && cc[j - 1] == '-' {
            j -= 2;
        } else if j != 2 * n - 1 && cc[j + 1] == '-' {
            j += 2;
        }
    }
    println!("{}", j / 2 + 1);
}
