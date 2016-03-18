Feature: Matching Steps

  Background:
    Given a new cucumber server

  Scenario: Trying to find an absent step
    Then the server cannot find a step matching "^example$"

  Scenario: Finding a matching step
    When I add a "Given" step definition with the regex "^example$"
    Then the server finds 1 step matching "example"

  Scenario: Finding several matching steps
    When I add a "Given" step definition with the regex "^example$"
    When I add a "Given" step definition with the regex "^e"
    Then the server finds 2 steps matching "example"
