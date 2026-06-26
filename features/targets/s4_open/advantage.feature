@row:advantage @stage:S4 @status:open @oracle:holospaces-cc @target
Feature: Advantage (TARGET — open; measured and reported, never asserted)
  # Whether content-addressed elision collapses cost below classical is a measurement.
  Scenario: content-reuse / elision is benchmarked and reported, never asserted
    Given a content-reuse benchmark probe
    Then the measured elision is recorded and advantage remains open and unasserted
