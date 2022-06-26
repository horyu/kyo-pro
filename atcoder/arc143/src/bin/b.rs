#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    if n == 1 {
        println!("0");
        return;
    }
    // 2 8 / 24
    // 3 254016 / 362880

    // 全体 - (列内MAX && 行内MIN)を満たす個数
    // 列内MAX => n以上
    // 行内MIN => nn - n + 1 以下
    let nn = n * n;
    let mut rs: ModInt = 1.into();
    for i in 1..=nn {
        rs *= i;
    }
    // dbg!(&rs);
    let mut mul = ModInt::from(1);
    for i in 1..=((n - 1).pow(2u32)) {
        mul *= i;
    }
    // dbg!(&mul);
    for x in n..=(nn - n + 1) {
        let mut sub = ModInt::from(1);
        // 列内の選び方 1..x から n-1 個選んで並べる
        for i in (1..x).rev().take(n - 1) {
            // eprintln!("tate {x} {i}");
            sub *= i;
        }
        // 行内の選び方 (x+1)..=nn から n-1 個選んで並べる
        for i in (1..=(nn - x)).rev().take(n - 1) {
            // eprintln!("yoko {x} {i}");
            sub *= i;
        }
        // 行列の縦横の場所
        sub *= nn;
        // (n-1)*(n-1)マスの並べ方
        sub *= mul;
        // eprintln!("{x} {}", sub.to_internal_num());
        rs -= sub;
    }
    println!("{}", rs.to_internal_num());
}

use crate::mod_int::ToInternalNum;
use mod_int::ModInt;
// https://github.com/kenkoooo/competitive-programming-rs/blob/master/src/math/mod_int.rs
pub mod mod_int {
    type ModInternalNum = i64;
    fn modulo() -> ModInternalNum {
        998244353
    }

    #[derive(Debug)]
    pub struct ModInt(ModInternalNum);
    impl Clone for ModInt {
        fn clone(&self) -> Self {
            Self(self.0)
        }
    }
    impl Copy for ModInt {}

    impl std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl ModInt {
        fn internal_new(mut v: ModInternalNum) -> Self {
            let m = modulo();
            if v >= m {
                v %= m;
            }
            Self(v)
        }

        pub fn internal_pow(&self, mut e: ModInternalNum) -> Self {
            let mut result = 1;
            let mut cur = self.0;
            let modulo = modulo();
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                    result %= modulo;
                }
                e >>= 1;
                cur = (cur * cur) % modulo;
            }
            Self(result)
        }

        pub fn pow<T>(&self, e: T) -> Self
        where
            T: ToInternalNum,
        {
            self.internal_pow(e.to_internal_num())
        }

        pub fn value(&self) -> ModInternalNum {
            self.0
        }
    }

    pub trait ToInternalNum {
        fn to_internal_num(&self) -> ModInternalNum;
    }
    impl ToInternalNum for ModInt {
        fn to_internal_num(&self) -> ModInternalNum {
            self.0
        }
    }
    macro_rules! impl_primitive {
        ($primitive:ident) => {
            impl From<$primitive> for ModInt {
                fn from(v: $primitive) -> Self {
                    let v = v as ModInternalNum;
                    Self::internal_new(v)
                }
            }
            impl ToInternalNum for $primitive {
                fn to_internal_num(&self) -> ModInternalNum {
                    *self as ModInternalNum
                }
            }
        };
    }
    impl_primitive!(u8);
    impl_primitive!(u16);
    impl_primitive!(u32);
    impl_primitive!(u64);
    impl_primitive!(usize);
    impl_primitive!(i8);
    impl_primitive!(i16);
    impl_primitive!(i32);
    impl_primitive!(i64);
    impl_primitive!(isize);

    impl<T: ToInternalNum> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }

            self.0 += rhs;
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }

    impl<T: ToInternalNum> std::ops::Add<T> for ModInt {
        type Output = ModInt;
        fn add(self, rhs: T) -> Self::Output {
            let mut res = self;
            res += rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            if rhs > 0 {
                self.0 += m - rhs;
            }
            if self.0 >= m {
                self.0 -= m;
            }
        }
    }
    impl<T: ToInternalNum> std::ops::Sub<T> for ModInt {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            let mut res = self;
            res -= rhs;
            res
        }
    }
    impl<T: ToInternalNum> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            self.0 *= rhs;
            self.0 %= m;
        }
    }
    impl<T: ToInternalNum> std::ops::Mul<T> for ModInt {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            let mut res = self;
            res *= rhs;
            res
        }
    }

    impl<T: ToInternalNum> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, rhs: T) {
            let mut rhs = rhs.to_internal_num();
            let m = modulo();
            if rhs >= m {
                rhs %= m;
            }
            let inv = Self(rhs).internal_pow(m - 2);
            self.0 *= inv.value();
            self.0 %= m;
        }
    }

    impl<T: ToInternalNum> std::ops::Div<T> for ModInt {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            let mut res = self;
            res /= rhs;
            res
        }
    }
}
