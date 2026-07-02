@row:archimedean-continuity @stage:S4 @status:some-true @oracle:mtc-axioms
Feature: Archimedean Continuity (Located)
  @row:archimedean-continuity
  Scenario: the continuous content of the coupled machine is located exactly
    Given the UOR Atlas use-case
    Then the archimedean continuity is exactly located on the 22-dim block
