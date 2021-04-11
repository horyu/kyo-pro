#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64,
    };
    let len = (x * x + y * y).sqrt();
    if len < r {
        println!("2");
    } else {
        println!("{}", ((x * x + y * y).sqrt() / r).ceil());
    }
}
