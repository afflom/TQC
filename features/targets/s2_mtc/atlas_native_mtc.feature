Feature: Atlas-native MTC construction
  @row:atlas-native-mtc
  Scenario: the Atlas-native construction is obstructed
    Given the UOR Atlas use-case
    Then the Atlas-native MTC construction returns an obstruction
