@row:solovay-kitaev @stage:S4 @status:some-true @oracle:mtc-axioms
Feature: Solovay-Kitaev Theorem
  Scenario: the representation is dense in SU(2)
    Given the UOR Atlas use-case
    Then the Solovay-Kitaev density proves epsilon-precision bounds in polynomial time
