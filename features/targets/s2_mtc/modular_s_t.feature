@row:modular-s-t @stage:S2 @status:build @oracle:mtc-axioms @target
Feature: Modular S/T matrices (TARGET — build, validated against axioms only, non-gating)
  Scenario: S and T satisfy the SL(2,Z) relations and Verlinde
    Given explicitly constructed modular S and T matrices from the theta-transformation
    Then S^4 = 1, (ST)^3 = S^2, S^2 = C, S is unitary and symmetric, and Verlinde holds
