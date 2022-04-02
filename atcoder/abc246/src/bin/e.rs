#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ax: Usize1,
        ay: Usize1,
        bx: Usize1,
        by: Usize1,
        ss: [Chars; n]
    };
    let mut qq = VecDeque::new();
    let mut pushed = vec![vec![false; n]; n];
    let mut viewed = vec![vec![[false; 4]; n]; n];
    qq.push_back((ax, ay, 0));
    pushed[ax][ay] = true;
    viewed[ax][ay] = [true; 4];
    while let Some((qx, qy, qi)) = qq.pop_front() {
        if qx == bx && qy == by {
            println!("{qi}");
            return;
        }
        if qx > 0 {
            // 左上
            for c in (1..n).take_while(|&c| c <= qx && c <= qy) {
                let px = qx - c;
                let py = qy - c;
                if ss[px][py] == '#' || viewed[px][py][0] {
                    break;
                }
                viewed[px][py][0] = true;
                if !pushed[px][py] {
                    pushed[px][py] = true;
                    qq.push_back((px, py, qi + 1));
                }
            }
            // 右上
            for c in (1..n).take_while(|&c| c <= qx && qy + c < n) {
                let px = qx - c;
                let py = qy + c;
                if ss[px][py] == '#' || viewed[px][py][1] {
                    break;
                }
                viewed[px][py][1] = true;
                if !pushed[px][py] {
                    pushed[px][py] = true;
                    qq.push_back((px, py, qi + 1));
                }
            }
        }
        if qx < n - 1 {
            // 左下
            for c in (1..n).take_while(|&c| qx + c < n && c <= qy) {
                let px = qx + c;
                let py = qy - c;
                if ss[px][py] == '#' || viewed[px][py][2] {
                    break;
                }
                viewed[px][py][2] = true;
                if !pushed[px][py] {
                    pushed[px][py] = true;
                    qq.push_back((px, py, qi + 1));
                }
            }
            // 右下
            for c in (1..n).take_while(|&c| qx + c < n && qy + c < n) {
                let px = qx + c;
                let py = qy + c;
                if ss[px][py] == '#' || viewed[px][py][3] {
                    break;
                }
                viewed[px][py][3] = true;
                if !pushed[px][py] {
                    pushed[px][py] = true;
                    qq.push_back((px, py, qi + 1));
                }
            }
        }
    }
    println!("-1");
}
