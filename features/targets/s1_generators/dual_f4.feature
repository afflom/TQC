@row:dual-f4 @stage:S1 @status:some-true @oracle:uor-addr-composition @target
Feature: Dual / conjugation reduces to compose_f4_quotient (TARGET — expected RED, non-gating)
  Scenario: the dual is the +/- mirror involution on every sigma-axis
    Given the tqc-substrate facade wired to uor-addr
    Then dual(a) equals compose_f4_quotient(a) and dual(dual(a)) equals a
