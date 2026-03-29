Feature: Apply unified diff

  Background:
    Given I have the llm-apply tool available

  # ── Happy paths ──────────────────────────────────────────────

  @iteration-1
  Scenario: Apply a single-hunk diff
    Given a file "src/main.rs" with content:
      """
      fn hello() {
          println!("hello");
      }
      """
    When I apply the unified diff:
      """
      --- a/src/main.rs
      +++ b/src/main.rs
      @@ -1,3 +1,3 @@
       fn hello() {
      -    println!("hello");
      +    println!("hello, world");
       }
      """
    Then the file contains:
      """
      fn hello() {
          println!("hello, world");
      }
      """
    And the exit code is 0

  @iteration-1
  Scenario: Apply a multi-hunk diff
    Given a file "src/lib.rs" with content:
      """
      pub fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      pub fn sub(a: i32, b: i32) -> i32 {
          a - b
      }
      """
    When I apply a diff that adds logging to both functions
    Then both functions contain a log statement
    And the exit code is 0

  @iteration-1
  Scenario: Dry run does not modify the file
    Given a file "src/main.rs" with content "fn foo() {}"
    When I apply a valid diff with --dry-run
    Then the file still contains "fn foo() {}"
    And the exit code is 0

  # ── Failure modes ─────────────────────────────────────────────

  @iteration-1
  Scenario: Hunk context does not match — fails loudly
    Given a file "src/main.rs" with content:
      """
      fn goodbye() {
          println!("bye");
      }
      """
    When I apply a diff that references "fn hello()"
    Then the exit code is 1
    And stderr contains "hunk did not match"
    And the file is unchanged

  @iteration-1
  Scenario: Empty change — fails with clear error
    Given a file "src/main.rs" with content "fn foo() {}"
    When I apply an empty change
    Then the exit code is 2
    And stderr contains "invalid input"

  # ── JSON output ───────────────────────────────────────────────

  @iteration-1
  Scenario: JSON output on success
    Given a file "src/main.rs" with valid content
    When I apply a valid diff with --json
    Then stdout is valid JSON
    And the JSON contains "success": true
    And the JSON contains a "hunks_applied" field
    And the JSON contains a "confidence" field

  @iteration-1
  Scenario: JSON output on failure
    Given a file "src/main.rs" with content "fn foo() {}"
    When I apply a non-matching diff with --json
    Then stdout is valid JSON
    And the JSON contains "success": false
    And the JSON contains an "error" field with an actionable message
