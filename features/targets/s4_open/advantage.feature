@row:advantage @stage:S4 @status:open @oracle:holospaces-cc @target
Feature: Advantage (open; measured and reported, never asserted)
  # Reframed as topological degeneracy: a braid's result depends only on its isotopy class, so
  # isotopic words address to one κ and elision computes the class once. This probe records the
  # degeneracy (braid paths per distinct result κ) only; it never asserts a speedup class.
  # Non-gating.
  Scenario: the topological degeneracy is measured
    Then the topological degeneracy is measured and advantage remains open and unasserted
