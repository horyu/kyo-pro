#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut h: String,
        mut bmi: String
    };
    h.retain(|c| c != '.');
    bmi.retain(|c| c != '.');
    let h = h.parse::<usize>().unwrap();
    let bmi = bmi.parse::<usize>().unwrap();
    println!("{}", (bmi * h.pow(2) / 10) as f64 / (1000.0f64).powi(2));
}
