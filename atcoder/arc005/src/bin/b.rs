#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: Isize1,
        y: Isize1,
        w: String,
        ccc: [Chars; 9]
    };
    let mut tate = if w.contains('U') {
        -1
    } else if w.contains('D') {
        1
    } else {
        0
    };
    let yy = (0..4).scan(y - tate, |state, _| {
        *state += tate;
        if *state < 0 {
            tate *= -1;
            *state = 1;
        } else if *state > 8 {
            tate *= -1;
            *state = 7;
        }
        Some(*state)
    });
    let mut yoko = if w.contains('L') {
        -1
    } else if w.contains('R') {
        1
    } else {
        0
    };
    let xx = (0..4).scan(x - yoko, |state, _| {
        *state += yoko;
        if *state < 0 {
            yoko *= -1;
            *state = 1;
        } else if *state > 8 {
            yoko *= -1;
            *state = 7;
        }
        Some(*state)
    });
    let rs = std::iter::zip(yy, xx)
        .map(|(y, x)| ccc[y as usize][x as usize])
        .join("");
    println!("{rs}");
}
