/-!
# Lagrange interpolation at x = 0

Aligns with `pss_lagrange.rs` / `lagrange_at_zero`.
-/

namespace PSS.Lagrange

/-- Abstract share point over a field carrier. -/
structure Point (α : Type) where
  x : α
  y : α

/-- Placeholder for Lagrange reconstruction at zero (Rust implements over GF256). -/
def lagrangeAtZero {α : Type} (_points : List (Point α)) : α :=
  sorry

theorem lagrange_at_zero_deterministic {α : Type} (pts : List (Point α)) :
    lagrangeAtZero pts = lagrangeAtZero pts := rfl

end PSS.Lagrange
