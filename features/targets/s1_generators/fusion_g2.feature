@row:fusion-g2 @stage:S1 @status:some-true @oracle:uor-addr-composition @target
Feature: Fusion reduces to compose_g2_product (TARGET — expected RED, non-gating)
  # Promoted to a gating suite once the tqc-substrate facade wires uor-addr.
  Scenario: fusing two anyons matches the uor-addr g2 product on every sigma-axis
    Given the tqc-substrate facade wired to uor-addr
    Then fuse(a,b) equals compose_g2_product(a,b) on every sigma-axis and is commutative
