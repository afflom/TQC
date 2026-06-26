@row:braiding-r-matrix @stage:S2 @status:build @oracle:mtc-axioms @target
Feature: Braiding R-matrix (TARGET — build, validated against axioms only, non-gating)
  # Constructed explicitly; validated against the universal MTC axioms; never assumed sound.
  Scenario: the R-matrix satisfies the hexagon identity and the Yang-Baxter equation
    Given an explicitly constructed braiding R-matrix over the generators and modular data
    Then R satisfies the hexagon identity and the Yang-Baxter equation for the full label set
