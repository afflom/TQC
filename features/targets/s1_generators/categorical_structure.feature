@row:categorical-structure @stage:S1 @status:some-true @oracle:uor-addr-composition @target
Feature: Categorical structure e6/e7/e8 (TARGET — expected RED, non-gating)
  Scenario: the e6/e7/e8 operations realize the grading, the S4 orbit and the embedding
    Given the tqc-substrate facade wired to uor-addr
    Then e6 yields the 8:1 grading, e7 the 24-element S4 orbit and e8 the E8 embedding
