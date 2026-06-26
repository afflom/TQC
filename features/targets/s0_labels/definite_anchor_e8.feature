@row:definite-anchor-e8 @stage:S0 @status:some-true @oracle:f1-atlas @target
Feature: Definite anchor — E8 Gram = 4x Cartan, PSD (TARGET — expected RED, non-gating)
  Scenario: the E8 Gram matrix is four times the Cartan matrix and is positive semidefinite
    Given the E8 root lattice realization
    Then the Gram matrix equals 4x Cartan (diag 8, edges -4) and is PSD as a sum of squares
