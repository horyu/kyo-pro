#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize
    };
    if x >= 90 {
        println!("expert");
    } else {
        let next = [40, 70, 90].iter().find(|&&n| n > x).unwrap();
        println!("{}", next - x);
    }
}
