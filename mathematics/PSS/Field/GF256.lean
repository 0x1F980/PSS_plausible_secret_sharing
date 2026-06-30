/-!
# GF(256) field sketch

Aligns with `pss_field_gf256.rs`: 256 elements, addition = XOR.
-/

namespace PSS.Field

abbrev GF256 := Fin 256

def add (a b : GF256) : GF256 :=
  ⟨((a.val ^^^ b.val) % 256), by
    have : (a.val ^^^ b.val) < 256 := by
      simp [Nat.xor_lt_two_pow]
      omega
    exact this⟩

theorem add_is_xor (a b : GF256) : add a b = ⟨a.val ^^^ b.val, by omega⟩ := rfl

theorem add_comm (a b : GF256) : add a b = add b a := by
  simp [add, Nat.xor_comm]

theorem add_assoc (a b c : GF256) : add (add a b) c = add a (add b c) := by
  simp [add, Nat.xor_assoc]

end PSS.Field
