#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        sss: [String; 3],
        t: Chars
    };
    let rs = t.into_iter().fold(String::new(), |mut s, c| {
        let c = c.to_digit(10).unwrap() as usize - 1;
        s.push_str(&sss[c]);
        s
    });
    println!("{}", rs);
}
