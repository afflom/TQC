@row:universality @stage:S4 @status:open @oracle:mtc-axioms @target
Feature: Universality (open; measured and reported, never asserted)
  # The generated-subgroup density is a genuine unknown. This probe records a measurement only;
  # it never asserts that the braiding is universal. Non-gating.
  Scenario: the generated-subgroup density is measured
    Then the generated-subgroup density is measured and universality remains open and unasserted
