@row:solovay-kitaev @stage:S4 @status:some-true @oracle:mtc-axioms
Feature: Solovay-Kitaev Density
  Scenario: the generated subgroup is measured and verified dense
    Given the UOR Atlas use-case
    Then the Solovay-Kitaev density bound is computationally established
