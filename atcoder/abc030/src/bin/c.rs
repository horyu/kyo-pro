#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [usize; 2],
        aa: [usize; n],
        bb: [usize; m]
    };
    let mut flight = 0;
    let mut from_to = &aa[..];
    let mut to_from = &bb[..];
    loop {
        if let Some(time) = from_to.first() {
            let now = time + xy[flight % 2];
            flight += 1;
            if let Some(left) = to_from.iter().position(|&t| t >= now) {
                to_from = &to_from[left..];
                std::mem::swap(&mut from_to, &mut to_from);
                continue;
            }
        }
        break;
    }
    println!("{}", flight / 2);
}
