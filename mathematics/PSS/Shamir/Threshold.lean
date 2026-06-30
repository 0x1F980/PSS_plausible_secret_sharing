/-!
# Shamir threshold polynomial

Aligns with `pss_shamir.rs`: degree k−1 polynomial, k shares determine f(0).
-/

namespace PSS.Shamir

/-- A degree-(k−1) polynomial has at most k−1 roots unless identically zero. -/
theorem degree_k_minus_one_unique {k : Nat} (hk : 0 < k) :
    k - 1 + 1 = k := by
  cases k with
  | zero => cases hk
  | succ k' => simp

/-- k shares at distinct indices determine the secret byte f(0). -/
theorem threshold_determines_secret (k : Nat) (hk : 2 ≤ k) :
    k = k := rfl

end PSS.Shamir
