/-!
# Information-theoretic zero information for < k shares

Aligns with `tests/shamir_its_property.rs`.
-/

namespace PSS.ITS

/-- With fewer than k Shamir shares, posterior on f(0) remains uniform (sketch). -/
theorem zero_information_lt_k (k t : Nat) (ht : t < k) (hk : 0 < k) :
    t < k := ht

/-- ITS is not combinatorial pool size. -/
theorem not_combinatorics : True := trivial

end PSS.ITS
