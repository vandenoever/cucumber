Feature: Beginning Scenarios

  Background:
    Given a new cucumber server

  Scenario: Starting a scenario with no tags
    When a scenario begins with the tags:
      | example-first  |
      | example-second |
    Then steps are run with the tags in context:
      | example-first  |
      | example-second |
