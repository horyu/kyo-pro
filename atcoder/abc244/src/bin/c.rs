#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{collections::*, io::Write};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    let n = s.trim_end().parse::<usize>().unwrap();
    let mut hs: HashSet<_> = (1..=(2 * n + 1)).collect();
    while let Some(&f) = hs.iter().next() {
        hs.remove(&f);
        println!("{}", f);
        stdout.flush().unwrap();
        if hs.is_empty() {
            return;
        }
        s.clear();
        stdin.read_line(&mut s).unwrap();
        let i = s.trim_end().parse::<usize>().unwrap();
        hs.remove(&i);
    }
}
