Feature: Retrieving snippets

  Background:
    Given a new cucumber server

  Scenario: Fetching snippet for a step
    When a snippet is requested for "Given" with the name "some missing step"
    Then a complete snippet is returned matching:
      """
      fn todo()
      """

