#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use std::collections::*;

use std::io::{stdin, stdout, Read, Write};
fn main() {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let _n = buf.trim_end().parse::<usize>().unwrap();
    buf.clear();
    // let mut v1 = vec![0; n + 1];
    // let mut v2 = vec![0; n + 1];
    // for i in 3..=n {
    //     println!("? 1 {}", i);
    //     stdout.flush().unwrap();
    //     stdin.read_line(&mut buf).unwrap();
    //     let d = buf.trim_end().parse::<usize>().unwrap();
    //     buf.clear();
    //     v1[i] = d;
    // }
    // for i in 3..=n {
    //     println!("? 2 {}", i);
    //     stdout.flush().unwrap();
    //     stdin.read_line(&mut buf).unwrap();
    //     let d = buf.trim_end().parse::<usize>().unwrap();
    //     buf.clear();
    //     v2[i] = d;
    // }
    // let mut rs = std::usize::MAX;
    // for i in 3..=n {
    //     rs = rs.min(v1[i] + v2[i]);
    // }
    // for i in 3..=n {
    //     if v1[i] + 1 == v2[i] || v1[i] == v2[i] + 1 {
    //         rs = 1;
    //     }
    // }
    // println!("! {}", rs);
    stdout.flush().unwrap();
}
