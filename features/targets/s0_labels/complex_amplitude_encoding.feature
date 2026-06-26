@row:complex-amplitude-encoding @stage:S0 @status:build @oracle:holospaces-cc @target
Feature: Complex amplitude encoding (TARGET — build, expected RED, non-gating)
  # The substrate stores bytes, not amplitudes; this is a defined content-addressed encoding.
  Scenario: C-coefficients over the labels round-trip through the content-addressed store
    Given the tqc-substrate facade wired to holospaces
    Then a fusion-space vector encodes to a kappa and decodes back byte-identically
