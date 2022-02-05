#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        t: usize,
        aass: [(usize, usize); t]
    };
    for (a, s) in aass {
        let mut tf = s >= 2 * a;
        if tf {
            tf = (s - 2 * a) & a == 0;
        }
        println!("{}", ["No", "Yes"][tf as usize]);
    }
}
