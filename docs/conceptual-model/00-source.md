# Holospaces TQC — Implementation Definition

> **This file is the conceptual authority.** It defines the TQC this repository realizes.
> Every other artifact — the typed model in `model/`, the BDD features, the V&V witnesses —
> derives from and must stay consistent with this document.

Status: living document; surgical edits.

What this defines: the realization, on the holospaces substrate, of the topological-quantum
(anyonic / modular-tensor-category) structure the UOR Atlas carries. A TQC in the
structural / simulation sense — not a physical anyonic device and not a claim of quantum
speedup. The MTC data splits into what the sources supply (objects, a genuine inner product,
fusion, conjugation, the spectrum, the coherence laws) and what is a defined build on top
(the braiding R-matrix, the modular S/T matrices, the complex amplitude encoding). Both are
tracked explicitly below; neither is asserted beyond what the sources show.

Sources: the [F1] Lean formalization of the UOR Atlas (`F1Square/Square/Atlas*`),
`UOR-Foundation/uor-addr` (the `composition` operations), and
`Hologram-Technologies/{hologram, holospaces}`. Claims without a source tag are design
decisions, marked as such.

Status vocabulary: `some-true` = a sourced fact reproduced (an F1 theorem, a realized
uor-addr operation, or a holospaces `vv` witness); `build` = a construction on sourced pieces,
validated against the universal MTC axioms; `open` = a genuine unknown, measured and reported,
never asserted true.

## Unitarity from orthogonality

The Atlas carries, on the 24-dim space `V_T ⊕ V_O` `(T,O)=(3,8)`, two distinct quadratic
forms that the construction keeps separate:

- the **balanced spectral operator** `M = (O+2)·I − T·Π_T − O·Π_O` — signature `(10,14)`,
  indefinite (`AtlasSpectrum`: `atlasM_signature`, `atlasM_indefinite`; spectrum
  `{10,7,2,−1}`, mults `{1,2,7,14}`). This is the superselection spectrum, not an inner
  product.
- the **definite Euclidean companion** `⟨x,x⟩ = Σxᵢ²` — positive-definite, a manifest sum of
  squares (`AtlasSpectrum` `WeilPSD_rankOne`; `AtlasCharacteristics` §5). This is the TQC
  inner product.

The TQC inner product is the Euclidean companion `Σxᵢ²`. The reflection generators `σ/τ/μ`
are coordinate permutations of the label space (below), hence orthogonal w.r.t. `Σxᵢ²`, hence
unitary — established directly. Separately, the **multiplicative composition norm**
`|x|²|y|² = |xy|²` (dims 1, 2, 4, 8, the octonion eight-square) is what makes fusion
norm-preserving (`AtlasComposition`, `some-true`). Braiding unitarity rides the Euclidean form
with no further assumption.

## TQC primitive — Atlas source (the dictionary)

| TQC primitive | Atlas / uor-addr realization | source | status |
|---|---|---|---|
| Objects (anyon labels) | byte ↔ (scope `q=2^{O−2T}=4`, modality `T=3`, context `O=8`); `96` classes, stride `T·O=24` | `AtlasClasses` §2 `classIndex`, `class_count_stride`, `classIndex_range` | some-true |
| Label / state-space index | the `12288 = 48×256 = 96×128` belt; `A_∞` inverse-limit address | `AtlasClasses` `belt_extent`; `AtlasAddressing` `atlas_parametric_generation` | some-true |
| Inner product (unitarity) | Euclidean definite companion `⟨x,x⟩=Σxᵢ²` on the 24-dim `V_T ⊕ V_O` | `AtlasSpectrum` `WeilPSD_rankOne`; `AtlasCharacteristics` §5 | some-true |
| Fusion `⊗` (commutative) | `compose_g2_product` → CS-G2 commutative binary product: orders the operand digests lex-min-first, concatenates `lo‖hi`, grounds through the σ-axis prism to a composed κ (commutativity structural); norm-multiplicative via the octonion 8-square | uor-addr `composition/g2`, `canonicalize_g2` (ADR-061/059); `AtlasComposition` `eight_square` | some-true |
| Dual / conjugation | `compose_f4` → CS-F4 ±mirror (2-element equivalence) = the Atlas mirror `μ` (order 2) | uor-addr `composition/f4`; `AtlasClasses` §3 `μ` | some-true |
| Categorical structure | `e6` (2-class 8:1 grading), `e7` (24-element S₄ orbit = the `T·O` orbit), `e8` (identity/embedding into E8) | uor-addr `composition/{e6,e7,e8}` (CS-E6/E7/E8) | some-true |
| Reflection generators | `σ` (order `q=4`), `τ` (order `O=8`), `μ` (order 2) — coordinate (class) permutations, orthogonal on `Σxᵢ²` | `AtlasClasses` §3 `sigma_order_four`, `rot` | some-true |
| Coxeter / Weyl group | E8 Coxeter `h=30`, exponents, `rank=φ(30)=8=O`; Weyl reflections | `AtlasCoxeter` `e8_coxeter_web`; `AtlasExceptional` `exceptional_dims` | some-true |
| Modular identities | `θ_{E8^T}=E4³=E6²+1728Δ`, `Δ=η²⁴`, weight `T·O/2=12` | `AtlasModular` `e4cube_eq_e6sq_plus_1728delta`, `twentyFour_modular` | some-true |
| Spectrum / superselection | `M` spectrum `{10,7,2,−1}`, mults `{1,2,7,14}`, the `−1`/G2 reflection (dim 14) | `AtlasSpectrum` `blockEig_spectrum`, `atlasMult` | some-true |
| Definite anchor (PSD seed) | E8 root lattice, Gram `= 4×` Cartan, PSD as SOS | `E8Seed` `e8_weilPSD`, `e8_is_cartan` | some-true |
| Ground space / protection | zero-state coherence: round-trip `π∘ι=id`, no-loss, scale-invariance | `AtlasCoherence` `atlas_coherent`; `vv` CC-1/2/29/30 | some-true |
| Braiding R-matrix | the braid datum (R/F satisfying hexagon / Yang–Baxter); built as the braiding of a representative pointed MTC (see Scope) | uor-addr / MTC axioms | build |
| Modular S/T matrices | the SL(2,ℤ) representation; built as the modular data of a representative pointed MTC (see Scope) | MTC axioms | build |
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
  permutations); the braiding R-matrix and the modular S/T matrices are the builds named in
  the dictionary.
- **Measurement** (fusion outcome / readout) is resolving the κ of the fused state. No-loss
  and the verify boundary are `CC-29` / `CC-30` (`restore(snapshot(m))` faithful inverse,
  byte-identical resume, κ-addressed migration).
- **Topological invariance** is the κ position-independence: the same content has the same
  address regardless of which tier or peer holds it; eviction drops bytes, not identity
  (`AtlasCoherence` no-loss; `CC-29`).
- **The whole TQC is a holospace** — a content-addressed compute artifact booted on the same
  peer that runs OS guests and (planned) the LLM, on one fabric.

## Scope: the realized braiding / modular layer

The braiding R-matrix and modular S/T (the `build` rows) are constructed as the modular data
of the **quantum double `D(Z_O)`** — `O = context` — an anomaly-free pointed modular tensor
category. This is a *representative* MTC realized at the Atlas parameters and validated
against the universal MTC axioms (SL(2,ℤ): `S` symmetric & unitary, `S⁴=1`, `(ST)³=S²`,
`S²=C`; hexagon; Yang–Baxter; Verlinde). It is **not** asserted to be the Atlas's own braided
category.

Two fusion structures are therefore carried, distinctly and deliberately:

- the **Atlas fusion** `⊗`, realized by `compose_g2_product` (the `fusion-g2` row) — the
  commutative content-addressed product on the label κ;
- the **representative MTC fusion** — the group law of `D(Z_O)` underlying its `S`/`T`/`R`.

These are different categories. Building the Atlas-native braided category — `S`/`T` from the
θ/Eisenstein modular data, `R` from `σ/τ/μ`, fusion from the `g2` rule, with the MTC axioms
proved on that identification — is **out of scope** here and recorded as such; the `build`
rows make no claim to it.

## Status ledger

- **some-true** (sourced — F1 theorem, realized uor-addr operation, or `vv` witness):
  objects/labels, the belt and `A_∞` address space, the Euclidean inner product, fusion
  (`g2`), conjugation (`f4`), the categorical operations (`e6/e7/e8`), the reflection
  generators (orthogonal/unitary), the Coxeter/Weyl group, the modular identities, the
  spectrum, the E8 PSD seed, the coherence/ground-space laws.
- **build** (construction on sourced pieces, validated against the universal MTC axioms only):
  - the **braiding R-matrix** — the braid datum satisfying hexagon / Yang–Baxter. Fusion
    (`g2`) is commutative, so braiding is extra data, not one of the composition operations.
  - the **modular S/T matrices** — the SL(2,ℤ) representation; the sources carry the modular
    identities, the build supplies the matrices.
  - the **complex amplitude encoding** — a content-addressed representation of ℂ-coefficients
    over the label index (the substrate stores bytes).
  See Scope for the representative-MTC framing of the first two.
- **open** (genuine unknowns, measured and reported, never asserted true):
  - **Universality** — whether the generators + braiding act densely in `U` of the fusion
    space (Freedman–Larsen–Wang). Measured (the generated braiding-phase order); reported.
  - **Advantage** — whether content-addressed elision (`content_reuse`, `cache_hits`)
    collapses the fusion-space computation below classical cost. Measured (the content-reuse
    ratio); reported. No verdict is asserted.

## Build stages

**S0 (labels + space + amplitudes).** Realize the `96`-class label set and the `12288` belt
as κ-addressed state in the holospaces store; build the complex amplitude encoding
(ℂ-coefficients over the labels) as a content-addressed map.
Exit: a state's κ is stable and re-derives (`CC-1` idiom).

**S1 (unitary generators + fusion).** Implement `σ/τ/μ`; verify they preserve `Σxᵢ²`
(orthogonal). Wire fusion to `compose_g2_product` and conjugation to `compose_f4`, calling the
realized operations rather than re-implementing them.
Exit: gate determinism witnessed (`CC-2` idiom); `Σxᵢ²` invariant under each generator;
fusion/dual reduce to the uor-addr operations.

**S2 (the MTC builds).** Construct the braiding R-matrix and the modular S/T matrices, and
validate them against the MTC axioms (hexagon / Yang–Baxter; SL(2,ℤ) / Verlinde) — see Scope.
Exit: the braiding satisfies hexagon and Yang–Baxter; S/T satisfy the SL(2,ℤ) relations.

**S3 (measurement + protection).** Fusion readout = κ-resolution; demonstrate
no-loss/round-trip (`CC-29`/`CC-30` idiom) as the topological-protection witness.
Exit: content-addressing round-trips with no loss.

**S4 (open questions, measured).** Probe universality (generated-subgroup density) and
elision advantage (`content_reuse`). Report; assert nothing the measurements don't show.
Exit: numbers recorded; universality and advantage stay `open` until established.

[F1]: https://github.com/afflom/F1
