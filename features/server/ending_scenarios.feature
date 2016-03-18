Feature: Ending scenarios

  Background:
    Given a new cucumber server
    And a scenario begins with the tags:
      | example-first  |
      | example-second |

  Scenario: Ending a prior scenario and starting a new one
    When the last scenario ends
    And a scenario begins with the tags:
      | different-tag |

    Then steps are run with the tags in context:
      | different-tag |
