#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [String; n],
        tt: [String; m],
    };
    let hs: HashSet<_> = HashSet::from_iter(tt.iter());
    for s in &ss {
        if hs.contains(s) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
