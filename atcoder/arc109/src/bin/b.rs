#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

fn main() {
    input! {
        n: usize,
    };
    if n == 1 {
        println!("{}", n);
        return;
    }
    for i in ((1 + 8 * n).sqrt() - 1).div(2)..=n {
        if (i + 1) * (i + 2) / 2 > n + 1 {
            println!("{}", n + 1 - i);
            return;
        }
    }
}
