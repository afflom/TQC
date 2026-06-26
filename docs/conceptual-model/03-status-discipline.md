# Status discipline

Inherited from the UOR Atlas / F1. Every dictionary row carries exactly one status. The
status is not a label of confidence; it is a **contract about what the V&V suite is allowed
to assert**. The honesty meta-gate enforces it mechanically.

| Status | Meaning | What V&V may assert | Where it lives |
|---|---|---|---|
| `some true` | Established: an F1 theorem, a realized uor-addr operation, or a runtime `vv` witness. | A green, gating check that the implementation reproduces the sourced fact. | `vv` gating witness (must pass) |
| `build` | A precisely-scoped construction on sourced pieces, crux-independent, not yet asserted complete. | That the construction satisfies the **universal axioms** (hexagon, Yang–Baxter, SL(2,ℤ), …) or reduces to a **sourced operation** — never that it is *the* unique or sound object. | gating against axioms only |
| `open` | A genuine unknown, not resolvable by decision (universality, advantage). | Only *measurements*. The claim itself is reported, never asserted true. | non-gating probe + report |
| `none` | The crux: the signed prime-form positivity (RH / `liPositivityHolds`). | **Nothing.** Neither assumed nor proved. | absent; verified absent |

## The three forms (why unitarity is not crux-gated)

The suite must never conflate:

1. **Euclidean `Σxᵢ²`** — the definite TQC inner product. `some true`. Generators are
   coordinate permutations, hence orthogonal w.r.t. this form, hence unitary. **No positivity
   assumption.**
2. **Multiplicative composition norm** `|x|²|y|²=|xy|²` (octonion 8-square) — what makes
   fusion norm-preserving. `some true`.
3. **Signed prime form** — the RH object. `none`. A *different* form (Atlas §9).

The no-smuggling check asserts the inner-product definition references only (1), so that
"braiding is unitary" is genuine orthogonality and not a disguised claim about (3).

## Forbidden assertions

The honesty gate fails CI if any witness or step asserts, as established:

- the RH crux / `liPositivityHolds` / signed-prime-form positivity (`none`);
- **universality** — density of the generated subgroup (`open`);
- **advantage** — sub-classical cost from content elision (`open`).

These may be *probed and reported*; their truth value is never green.
