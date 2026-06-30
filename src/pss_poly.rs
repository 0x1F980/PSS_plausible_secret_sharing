// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_field_gf256::Gf256;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct Polynomial {
    pub coeffs: alloc::vec::Vec<Gf256>,
}

impl Polynomial {
    pub fn new(coeffs: alloc::vec::Vec<Gf256>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate(&self, x: Gf256) -> Gf256 {
        let mut result = Gf256::ZERO;
        for &c in self.coeffs.iter().rev() {
            result = result * x + c;
        }
        result
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poly_eval() {
        let p = Polynomial::new(alloc::vec![Gf256(5), Gf256(3)]);
        assert_eq!(p.evaluate(Gf256(0)), Gf256(5));
        assert_eq!(p.evaluate(Gf256(2)), Gf256(3) * Gf256(2) + Gf256(5));
    }
}
