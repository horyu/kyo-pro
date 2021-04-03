#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let rs = s.iter().zip(t.iter()).filter(|(sc, tc)| sc != tc).count();
    println!("{}", rs);
}
