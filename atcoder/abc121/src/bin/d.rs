#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    if a == b {
        println!("{}", a);
        return;
    }

    let f = |x| match x % 4 {
        0 => x,
        1 => 1,
        2 => x ^ 1,
        3 => 0,
        _ => unreachable!(),
    };
    println!("{}", f(a.saturating_sub(1)) ^ f(b));
}

const POW2: [usize; 64] = {
    let mut arr = [0usize; 64];
    let mut i = 0u32;
    while i < 64 {
        arr[i as usize] = 2usize.pow(i);
        i += 1;
    }
    arr
};

fn main2() {
    input! {
        a: usize,
        b: usize,
    };
    if a == b {
        println!("{}", a);
        return;
    }

    // 0-(a-1), 0-b までのオンビットの個数を数える
    let f = |x: usize| {
        let mut arr = [0usize; 63];
        for (i, ar) in arr.iter_mut().enumerate() {
            // 2**(i + 1) の周期の数 * 2**i
            *ar += x / POW2[i + 1] * POW2[i];
            // 周期の途中の場合、何番目か計算
            if x & POW2[i] > 0 {
                *ar += (x & POW2[i].saturating_sub(1)) + 1;
            }
        }
        arr
    };
    let mut num = 0;
    let a = a.saturating_sub(1);
    for (i, (bc, ac)) in f(b).iter().zip(f(a).iter()).enumerate() {
        if (bc - ac) % 2 == 1 {
            num += POW2[i];
        }
    }
    println!("{}", num);
}
