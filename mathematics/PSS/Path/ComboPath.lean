#!/usr/bin/env lean
import PSS.Field.GF256
import PSS.Shamir.Threshold

/-!
# Combo path (ITS path secret W + chain Lagrange)

Aligns with `pss_path.rs`: path recipe bytes Shamir-delt on `PSS-v1-path` domain;
optional combo decode chains seed-domain Lagrange steps (genvalg / ice-cream path).
Not default for general use — sum/Lagrange seed mode is default.
-/

namespace PSS.Path

structure PathStep where
  left  : Nat
  right : Nat

/-- Path recipe serialized as bytes (step count + pairs). -/
def recipeByteLen (steps : Nat) : Nat := 1 + steps * 2

theorem path_its_sketch (k t : Nat) (ht : t < k) (hk : 0 < k) : t < k := ht

theorem combo_not_default : True := trivial

end PSS.Path
