#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut ww: [String; n]
    };
    ww.last_mut().unwrap().pop().unwrap();
    let rs = ww
        .iter()
        .filter(|s| matches!(s.as_ref(), "TAKAHASHIKUN" | "Takahashikun" | "takahashikun"))
        .count();
    println!("{}", rs);
}
