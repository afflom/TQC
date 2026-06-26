@row:ground-space-protection @stage:S3 @status:some-true @oracle:holospaces-cc @target
Feature: Ground space / topological protection (TARGET — expected RED, non-gating)
  Scenario: a TQC holospace round-trips with no loss
    Given the tqc-substrate facade wired to holospaces
    Then restore(snapshot(m)) reproduces the state byte-identically (CC-29/CC-30)
    And the same content resolves to the same kappa regardless of tier or peer
