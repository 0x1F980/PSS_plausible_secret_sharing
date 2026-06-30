// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

use crate::pss_field_gf256::Gf256;
use subtle::{Choice, ConditionallySelectable};

/// Lagrange interpolation at `x` over GF(256).
pub fn lagrange_interpolate(points: &[(Gf256, Gf256)], x: Gf256) -> Gf256 {
    let mut result = Gf256::ZERO;
    let n = points.len();

    for i in 0..n {
        let mut numerator = Gf256::ONE;
        let mut denominator = Gf256::ONE;

        for j in 0..n {
            let is_different = Choice::from((i != j) as u8);
            let term_num = x - points[j].0;
            let term_den = points[i].0 - points[j].0;

            let num_factor = Gf256::conditional_select(&Gf256::ONE, &term_num, is_different);
            let den_factor = Gf256::conditional_select(&Gf256::ONE, &term_den, is_different);

            numerator = numerator * num_factor;
            denominator = denominator * den_factor;
        }

        let basis = numerator * denominator.inv();
        result = result + points[i].1 * basis;
    }

    result
}

/// Evaluate at x=0 (recover secret byte from shares).
pub fn lagrange_at_zero(points: &[(Gf256, Gf256)]) -> Gf256 {
    lagrange_interpolate(points, Gf256::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lagrange_two_points() {
        // f(x) = 5 + 3x over GF(256): f(1)=6, f(2)=3, f(0)=5
        let points = [(Gf256(1), Gf256(6)), (Gf256(2), Gf256(3))];
        assert_eq!(lagrange_at_zero(&points), Gf256(5));
        assert_eq!(lagrange_interpolate(&points, Gf256(2)), Gf256(3));
    }
}
