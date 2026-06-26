@row:advantage @stage:S4 @status:open @oracle:holospaces-cc @target
Feature: Advantage (open; measured and reported, never asserted)
  # Whether content-addressed elision collapses cost below classical is a measurement. This
  # probe records the content-reuse ratio only; it never asserts a speedup class. Non-gating.
  Scenario: the content-reuse ratio is measured
    Then the content-reuse ratio is measured and advantage remains open and unasserted
