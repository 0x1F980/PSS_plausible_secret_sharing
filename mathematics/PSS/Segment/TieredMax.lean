/-!
# Tiered maximum secret size

Aligns with `pss_segment.rs` / `pss_capacity.rs`:
  |F*|_max = L_{n−k+1} for sorted carrier sizes L₁ ≤ … ≤ Lₙ.
-/

namespace PSS.Segment

def sortedSize (sizes : List Nat) : List Nat :=
  sizes.mergeSort (fun a b => a ≤ b)

/-- Index n−k+1 in 1-based sorted list (when defined). -/
def tierIndex (n k : Nat) : Nat :=
  n - k + 1

/-- Maximum contiguous secret length (sketch). -/
def maxSecretSize (sizes : List Nat) (k n : Nat) (hk : k ≤ n) (hn : n ≤ sizes.length) : Nat :=
  let sorted := sortedSize sizes
  let idx := tierIndex n k
  sorted.get! (idx - 1)

theorem tiered_max_example :
    maxSecretSize [2, 3, 4, 5, 6] 3 5 (by omega) (by omega) = 4 := by
  native_decide

end PSS.Segment
