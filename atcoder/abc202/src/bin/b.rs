#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    let s = s
        .into_iter()
        .rev()
        .map(|c| match c {
            // '0' => '0',
            // '1' => '1',
            '6' => '9',
            // '8' => '8',
            '9' => '6',
            _ => c,
        })
        .collect::<String>();
    println!("{}", s);
}
