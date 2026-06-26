# Holospaces TQC — Implementation Definition

> **This file is the cited authority.** It is the source specification this repository
> realizes. Notation has been restored from the originally-supplied document; semantics
> are preserved verbatim. Every other artifact in this repository — the typed model in
> `model/`, the BDD features, the V&V witnesses — derives from and must stay consistent
> with this document.

Status: DRAFT v0.3. Living document; surgical edits, never regenerated.

What this defines: the realization, on the holospaces substrate, of the topological-quantum
(anyonic / modular-tensor-category) structure the UOR Atlas carries. A TQC in the
structural / simulation sense — not a physical anyonic device and not a claim of quantum
speedup. The MTC data splits into what the source supplies (objects, a genuine inner
product, fusion, conjugation, the spectrum, the coherence laws) and what is a defined build
on top (the braiding R-matrix, the modular S/T matrices, the complex amplitude encoding).
Both are tracked explicitly below; neither is asserted beyond what source shows.

Verification basis: source read @ depth-1 HEAD of `afflom/F1` (Atlas formalization,
`F1Square/Square/Atlas*`), `UOR-Foundation/uor-addr` (the `composition` operations),
`Hologram-Technologies/{hologram, holospaces}`. F1 is the sourced authority; its open crux
is respected, not re-litigated. Claims without a source tag are design decisions, marked as
such.

Convention (inherited from the Atlas): `some true` = established (a theorem in F1, a
realized operation in uor-addr, or a runtime `vv` witness); `build` = a precisely-scoped
construction on sourced pieces, not yet built; `none` = open, never asserted. The crux
stays `none`.

## The unlock: unitarity is not crux-gated

The reason a TQC can be defined here without touching RH is one fact, sourced. The Atlas
carries, on the 24-dim space `V_T ⊕ V_O` `(T,O)=(3,8)`, two quadratic forms:

- the **balanced spectral operator** `M = (O+2)·I − T·Π_T − O·Π_O`, signature `(10,14)`,
  indefinite (`AtlasSpectrum`: `atlasM_signature`, `atlasM_indefinite`; spectrum
  `{10,7,2,−1}`, mults `{1,2,7,14}`; positive eigenspaces sum `38`, the `−1` reflection
  nets `38−14 = T·O = 24`, `AtlasCharacteristics` §5);
- its **definite Euclidean companion** `⟨x,x⟩ = Σxᵢ²`, positive-definite, a manifest sum of
  squares (`AtlasSpectrum` §9 `WeilPSD_rankOne`).

These are different objects, and F1 records (faithfully to the Atlas §9) that the definite
one is **not** the signed prime form whose non-negativity is RH. So three forms must not be
conflated: the Euclidean `Σxᵢ²` (the TQC inner product, definite, `some true`); the
multiplicative composition norm at the tower levels `|x|²|y|²=|xy|²` (dims 1,2,4,8, what
makes fusion norm-preserving, `AtlasComposition`, `some true`); and the signed prime form
(RH, `none`). The braid generators are coordinate permutations of the label space (below),
hence orthogonal, hence unitary w.r.t. `Σxᵢ²` with no positivity assumption. So braiding
unitarity is `some true`; RH positivity is `none`; the construction does not depend on the
crux.

## TQC primitive — Atlas source (the dictionary)

| TQC primitive | Atlas / uor-addr realization | source | status |
|---|---|---|---|
| Objects (anyon labels) | byte ↔ (scope `q=2^{O−2T}=4`, modality `T=3`, context `O=8`); `96` classes, stride `T·O=24` | `AtlasClasses` §2 `classIndex`, `class_count_stride`, `classIndex_range` | some true |
| Label / state-space index | the `12288 = 48×256 = 96×128` belt; `A_∞` inverse-limit address | `AtlasClasses` `belt_extent`; `AtlasAddressing` `atlas_parametric_generation` | some true |
| Inner product (unitarity) | Euclidean definite companion `⟨x,x⟩=Σxᵢ²` on the 24-dim `V_T ⊕ V_O` | `AtlasSpectrum` §9 `WeilPSD_rankOne`; `AtlasCharacteristics` §5 | some true |
| Fusion `⊗` (commutative) | `compose_g2_product` → CS-G2 commutative binary product: orders the operand digests lex-min-first, concatenates `lo‖hi`, grounds through the σ-axis prism to a composed κ (commutativity structural); norm-multiplicative via the octonion 8-square | uor-addr `composition/g2`, `canonicalize_g2` (ADR-061/059); `AtlasComposition` `eight_square` | some true |
| Dual / conjugation | `compose_f4` → CS-F4 ±mirror (2-element equivalence) = the Atlas mirror `μ` (order 2) | uor-addr `composition/f4`; `AtlasClasses` §3 `μ` | some true |
| Categorical structure | `e6` (2-class 8:1 grading), `e7` (24-element S₄ orbit = the `T·O` orbit), `e8` (identity/embedding into E8) | uor-addr `composition/{e6,e7,e8}` (CS-E6/E7/E8) | some true |
| Reflection generators | `σ` (order `q=4`), `τ` (order `O=8`), `μ` (order 2) — coordinate (class) permutations, orthogonal on `Σxᵢ²` | `AtlasClasses` §3 `sigma_order_four`, `rot` | some true |
| Coxeter / Weyl group | E8 Coxeter `h=30`, exponents, `rank=φ(30)=8=O`; Weyl reflections | `AtlasCoxeter` `e8_coxeter_web`; `AtlasExceptional` `exceptional_dims` | some true |
| Modular identities | `θ_{E8^T}=E4³=E6²+1728Δ`, `Δ=η²⁴`, weight `T·O/2=12` | `AtlasModular` `e4cube_eq_e6sq_plus_1728delta`, `twentyFour_modular` | some true |
| Spectrum / superselection | `M` spectrum `{10,7,2,−1}`, mults `{1,2,7,14}`, the `−1`/G2 reflection (dim 14) | `AtlasSpectrum` `blockEig_spectrum`, `atlasMult` | some true |
| Definite anchor (PSD seed) | E8 root lattice, Gram `= 4×` Cartan, PSD as SOS | `E8Seed` `e8_weilPSD`, `e8_is_cartan` | some true |
| Ground space / protection | zero-state coherence: round-trip `π∘ι=id`, no-loss, scale-invariance | `AtlasCoherence` `atlas_coherent`; `vv` CC-2/29/30 | some true |
| Braiding R-matrix | the non-commutative braid datum (R/F satisfying hexagon / Yang–Baxter) over the reflection generators + modular data | — (not in source) | build |
| Modular S/T matrices | the SL(2,ℤ) representation on the characters from the θ-transformation | — (only the identities are in source) | build |
| Complex amplitude encoding | a content-addressed representation of ℂ-coefficients over the label index | — (the substrate stores bytes, not amplitudes) | build |

## Substrate realization (how each row runs on holospaces)

- **State** is content, but the amplitude layer is a build. The substrate provides the label
  index (a class κ) and content-addressed storage in the uniform `A_∞` store
  (`MemKappaStore`→OPFS→peer, one σ-axis keyspace, no RAM/OPFS boundary since the address is
  the content hash). A fusion-space vector — ℂ-coefficients over the labels — is a defined
  encoding on top of that storage (the amplitude-encoding build); the substrate stores
  bytes, not amplitudes.
- **Gates** are `.holo` compute artifacts run by the native `.holo` Engine
  (`hologram_exec::InferenceSession`, `holospaces/crates/holospaces/src/engine.rs`), with
  determinism — identical gate + state → identical output κ — witnessed by `CC-2`
  (`cc2_holo_engine`). The reflection generators `σ/τ/μ` are realized directly (coordinate
  permutations); the braiding R-matrix and the modular S/T matrices they compose with are
  the builds named in the dictionary, not present in source.
- **Measurement** (fusion outcome / readout) is resolving the κ of the fused state. No-loss
  and the verify boundary are `CC-29` / `CC-30` (`restore(snapshot(m))` faithful inverse,
  byte-identical resume, κ-addressed migration).
- **Topological invariance** is the κ position-independence: the same content has the same
  address regardless of which tier or peer holds it; eviction drops bytes, not identity
  (`AtlasCoherence` no-loss; `CC-29`).
- **The whole TQC is a holospace** — a content-addressed compute artifact booted on the same
  peer that runs OS guests and (planned) the LLM, on one fabric, witnessed across three ISAs
  (`vv` `cc43/44/45`, arch-parity `cc46`).

## Status ledger

- **some true** (sourced — F1 theorem, realized uor-addr operation, or `vv` witness):
  objects/labels, the belt and `A_∞` address space, the Euclidean inner product, fusion
  (`g2`), conjugation (`f4`), the categorical operations (`e6/e7/e8`), the reflection
  generators (orthogonal/unitary), the Coxeter/Weyl group, the modular identities, the
  spectrum, the E8 PSD seed, the coherence/ground-space laws.
- **build** (precisely-scoped construction on sourced pieces, not yet built,
  crux-independent):
  - the **braiding R-matrix** — the non-commutative braid datum over the reflection
    generators and modular data (hexagon / Yang–Baxter). Fusion (`g2`) is commutative, so
    braiding is extra data, not one of the composition operations.
  - the **modular S/T matrices** — the SL(2,ℤ) representation on the characters from the
    θ-transformation; source carries the modular identities, not the matrices.
  - the **complex amplitude encoding** — a content-addressed representation of ℂ-coefficients
    over the label index (the substrate stores bytes).
  These three are the work to assemble the MTC from the sourced pieces. None needs the crux.
- **open** (genuine unknowns, not buildable by decision):
  - **Universality** — whether the generators + braiding act densely in `U` of the fusion
    space (Freedman–Larsen–Wang). Not in source; until shown, computational power is unknown.
  - **Advantage** — whether content-addressed elision (`ADR-041`, `content_reuse`,
    `cache_hits`) collapses the fusion-space computation below classical cost. A measurement;
    classical simulation of a quantum state space is generally exponential, and elision helps
    exactly where content repeats. Routed to a `vv` `perf`/`content_reuse` witness; no
    verdict pre-bench.
- **none** (the crux, untouched and independent): the signed prime-form positivity (RH /
  `liPositivityHolds`) is a different form (Atlas §9). This construction neither assumes nor
  proves it.

## Build stages

**S0 (labels + space + amplitudes).** Realize the `96`-class label set and the `12288` belt
as κ-addressed state in the holospaces store; build the complex amplitude encoding
(ℂ-coefficients over the labels) as a content-addressed map.
Exit: a state's κ is stable and re-derives (`CC-1` idiom).

**S1 (unitary generators + fusion).** Implement `σ/τ/μ` as `.holo` ops; verify they preserve
`Σxᵢ²` (orthogonal) against the native executor oracle. Wire fusion to
`compose_g2_product` (per σ-axis: it lex-min-first orders, concatenates, and grounds the
operand digests) and conjugation to `compose_f4`, calling the realized operations rather than
re-implementing them.
Exit: gate determinism witnessed (`CC-2` idiom); `Σxᵢ²` invariant under each generator;
fusion/dual reduce to the uor-addr operations.

**S2 (the MTC builds).** Construct the braiding R-matrix over the generators + modular data
(hexagon / Yang–Baxter), and the modular S/T matrices from the θ-transformation. These are
the three `build` rows.
Exit: a braid word applied to a state yields a content-addressed result satisfying
Yang–Baxter; S/T satisfy the realized modular identities.

**S3 (measurement + protection).** Fusion readout = κ-resolution; demonstrate
no-loss/round-trip (`CC-29`/`CC-30` idiom) as the topological-protection witness.
Exit: a full braid → fuse → read cycle runs as one holospace, resumable.

**S4 (open questions, measured).** Probe universality (generated-subgroup density) and
elision advantage (`content_reuse`/`perf`). Report; assert nothing the benches don't show.
Exit: numbers recorded; universality and advantage stay `none` until established; the crux
stays `none`.
