@row:finite-closure @stage:S4 @status:some-true @oracle:mtc-axioms
Feature: Finite-closure representation
  Finite-closure is the closure of the equivalency facet: the set of distinct κ classes generated
  by the braiding is mathematically finite. It is the size of the universal equivalence.
  Scenario: the generated subgroup is mathematically finite
    Given the UOR Atlas use-case
    Then the generated subgroup is proven mathematically finite precluding density
