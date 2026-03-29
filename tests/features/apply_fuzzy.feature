Feature: Fuzzy matching for drifted files

  Background:
    Given I have the llm-apply tool available

  # ── Drift levels ──────────────────────────────────────────────

  @iteration-4
  Scenario: Apply change when trailing whitespace was removed (L1 drift)
    Given the LLM generated a change for a file with trailing spaces
    And the file has since been saved without trailing spaces
    When I apply the change with fuzzy matching enabled
    Then the change is applied correctly
    And the confidence score is greater than 0.90

  @iteration-4
  Scenario: Apply change when rustfmt changed indentation (L1 drift)
    Given a file formatted with 2-space indentation
    And a change generated from that file
    And the file has since been reformatted with 4-space indentation
    When I apply the change
    Then the change is applied correctly

  @iteration-4
  Scenario: Apply change when type hints were added nearby (L2 drift)
    Given a Python file with untyped function signatures
    And a change that modifies the body of a function
    And the file has since had type hints added to the signatures
    When I apply the change
    Then the function body is updated correctly
    And the type hints are preserved

  @iteration-4
  Scenario: Apply change when a comment was added near the target (L3 drift)
    Given a file with a function
    And a change that modifies that function
    And the file has since had a docstring added to the function
    When I apply the change
    Then the function body is updated
    And the docstring is preserved

  @iteration-4
  Scenario: Apply change when a new function was added nearby (L4 drift)
    Given a file with two functions
    And a change targeting the second function
    And the file has since had a third function inserted between them
    When I apply the change
    Then the correct function is modified
    And the new function is untouched

  # ── Confidence threshold ──────────────────────────────────────

  @iteration-4
  Scenario: Low confidence — refuses to apply and reports score
    Given a file that has been substantially refactored
    When I apply a change from the old version
    Then the exit code is 1
    And the JSON output contains a confidence score below 0.5
    And stderr suggests using --force or regenerating the change

  @iteration-4
  Scenario: Force flag applies even with low confidence
    Given a file that has been substantially refactored
    When I apply a change from the old version with --force
    Then the exit code is 0
    And stderr warns about low confidence

  # ── Ambiguity ─────────────────────────────────────────────────

  @iteration-2 @iteration-4
  Scenario: Search block matches multiple locations — refuses and reports candidates
    Given a file with this pattern appearing three times:
      """
      if user is None:
          return None
      """
    When I apply a change targeting that pattern
    Then the exit code is 1
    And stderr contains "ambiguous match"
    And stderr reports the line numbers of all three occurrences
    And the file is unchanged
