// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use core::ops::{Add, Mul, Neg, Sub};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zeroize::Zeroize;

static EXP: [u8; 256] = build_exp();
static LOG: [u8; 256] = build_log();

const fn build_exp() -> [u8; 256] {
    let mut exp = [0u8; 256];
    let mut x = 1u8;
    let mut i = 0usize;
    while i < 255 {
        exp[i] = x;
        x = gf256_mul_const(x, 3);
        i += 1;
    }
    exp[255] = 0;
    exp
}

const fn build_log() -> [u8; 256] {
    let exp = build_exp();
    let mut log = [0u8; 256];
    let mut i = 0usize;
    while i < 255 {
        log[exp[i] as usize] = i as u8;
        i += 1;
    }
    log[0] = 0;
    log
}

const fn gf256_mul_const(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    let mut i = 0;
    while i < 8 {
        if b & 1 != 0 {
            p ^= a;
        }
        let hi = a & 0x80;
        a <<= 1;
        if hi != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
        i += 1;
    }
    p
}

/// Element of GF(2^8) with reduction polynomial x^8 + x^4 + x^3 + x + 1.
#[derive(Clone, Copy, Debug, Default, Zeroize, PartialEq, Eq)]
pub struct Gf256(pub u8);

impl Gf256 {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);

    #[inline]
    pub fn new(val: u8) -> Self {
        Self(val)
    }

    #[inline]
    pub fn from_u8(val: u8) -> Self {
        Self(val)
    }

    #[inline]
    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn inv(self) -> Self {
        if self.0 == 0 {
            return Self::ZERO;
        }
        let e = 255u16 - LOG[self.0 as usize] as u16;
        Self(EXP[(e % 255) as usize])
    }

    pub fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl Add for Gf256 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Sub for Gf256 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Neg for Gf256 {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

impl Mul for Gf256 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if self.0 == 0 || rhs.0 == 0 {
            return Self::ZERO;
        }
        let log_sum = LOG[self.0 as usize] as u16 + LOG[rhs.0 as usize] as u16;
        Self(EXP[(log_sum % 255) as usize])
    }
}

impl ConditionallySelectable for Gf256 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(u8::conditional_select(&a.0, &b.0, choice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gf256_add_is_xor() {
        assert_eq!(Gf256(0x57) + Gf256(0x83), Gf256(0x57 ^ 0x83));
    }

    #[test]
    fn gf256_mul_inverse() {
        let a = Gf256(0x53);
        assert_eq!(a * a.inv(), Gf256::ONE);
    }

    #[test]
    fn gf256_aes_known_vector() {
        assert_eq!(Gf256(0x57) * Gf256(0x83), Gf256(0xc1));
    }

    #[test]
    fn lagrange_known_gf256() {
        use crate::pss_lagrange::lagrange_at_zero;
        let shares = [(1u8, Gf256(6)), (2u8, Gf256(3))];
        assert_eq!(lagrange_at_zero(&shares.map(|(i, v)| (Gf256::from_u8(i), v))), Gf256(5));
    }
}
