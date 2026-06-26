@row:universality @stage:S4 @status:open @oracle:mtc-axioms @target
Feature: Universality (TARGET — open; measured and reported, never asserted)
  # The generated-subgroup density is a genuine unknown. The probe records a measurement only.
  Scenario: the density of the generated subgroup is measured and reported, never asserted
    Given a generated-subgroup density probe
    Then the measured density is recorded and universality remains open and unasserted
