# Simple Tests & Core Unit Tests Report (`mpc-rs`)

## 1. Executive Summary & Execution Metadata

This report summarizes the execution results for the smoke example suite in [`examples/simple_test.rs`](../examples/simple_test.rs) alongside all **21 unit tests** in [`src/lib.rs`](../src/lib.rs) across both **Debug** and **Release** profiles.

| Property | Value |
|---|---|
| **Repository / Commit** | `Mr-Kumar-Abhishek/mpc-rs` (`810e387`) |
| **Targets Tested** | [`examples/simple_test.rs`](../examples/simple_test.rs) + unit suite in [`src/lib.rs`](../src/lib.rs) |
| **CI Workflow / Run ID** | [`.github/workflows/rust.yml`](../.github/workflows/rust.yml) — Run `#44799266904` (`success`) |
| **Runner OS** | `ubuntu-latest` (GitHub Actions) |
| **Overall Result** | **`5/5` `simple_test` smoke checks verified** and **`21/21` unit tests passed** in both Debug and Release builds |

---

## 2. Smoke Example Suite (`examples/simple_test.rs`)

### 2.1 Case-by-Case Summary

| # | Scenario | Parser / Combinator | Input | Debug & Release Outcome |
|---|---|---|---|---|
| 1 | Single character match | `mpc_char('a')` | `"a"` | Pass — printed `Success: Some("a")` |
| 2 | Exact string match | `mpc_string("hello")` | `"hello"` | Pass — printed `Success: Some("hello")` |
| 3 | Expected error formatting | `mpc_char('a')` | `"b"` | Pass — printed `test:1:1: error: expected 'a' (expected a, received 'b')` |
| 4 | Choice fallback (`or`) | `mpc_or(vec![mpc_char('a'), mpc_char('b')])` | `"b"` | Pass — printed `Or Success: Some("b")` |
| 5 | Recursive AST construction | `mpca_root` + `mpca_tag` + `mpcf_fold_ast` | `"12+34"` | Pass — printed `>\|expr` tree with `number("12")`, `op("+")`, `number("34")` |

### 2.2 Captured CI Output (Debug & Release)

Both `cargo run --example simple_test` and `cargo run --release --example simple_test` in Run `#44799266904` exited with status `0` and produced identical output:

```text
Success: Some("a")
Success: Some("hello")
test:1:1: error: expected 'a' (expected a, received 'b')
Or Success: Some("b")
>|expr
  number
    "12"
  op
    "+"
  number
    "34"
```

---

## 3. Core Unit Test Suite (`src/lib.rs` — 21 Tests)

All 21 unit tests passed in `0.00s` under both `cargo test --verbose` (Debug) and `cargo test --release --verbose` (Release):

| Category | Test Function | Debug | Release |
|---|---|---:|---:|
| **Primitives** | `basic_char_any_range_oneof_primitives` | Pass | Pass |
| **Primitives** | `noneof_matches_char_and_fails_at_end_of_input` | Pass | Pass |
| **Primitives** | `satisfy_failure_does_not_advance_input` | Pass | Pass |
| **Primitives** | `string_partial_match_backtracks` | Pass | Pass |
| **Primitives** | `control_parsers_pass_fail_lift_and_state` | Pass | Pass |
| **Combinators & Folds** | `or_backtracks_after_partial_branch_failure` | Pass | Pass |
| **Combinators & Folds** | `many_and_many1_handle_repetitions_and_zero_width_guard` | Pass | Pass |
| **Combinators & Folds** | `sepby_and_sepby1_backtrack_trailing_separator` | Pass | Pass |
| **Combinators & Folds** | `fold_functions_and_character_utilities` | Pass | Pass |
| **UTF-8 Cursor** | `utf8_input_tracks_row_col_and_byte_pos` | Pass | Pass |
| **Anchors & Rollback** | `soi_anchor_matches_only_at_start_of_input` | Pass | Pass |
| **Anchors & Rollback** | `eoi_anchor_matches_only_at_end_of_input` | Pass | Pass |
| **Anchors & Rollback** | `boundary_newline_anchor_checks_non_word_transitions` | Pass | Pass |
| **Anchors & Rollback** | `and_restores_input_state_when_later_parser_fails` | Pass | Pass |
| **Anchors & Rollback** | `count_restores_input_state_when_later_iteration_fails` | Pass | Pass |
| **Token & Literal Parsers** | `int_hex_oct_and_number_parsers` | Pass | Pass |
| **Token & Literal Parsers** | `real_and_float_parsers` | Pass | Pass |
| **Token & Literal Parsers** | `char_string_and_regex_literal_parsers` | Pass | Pass |
| **Token & Literal Parsers** | `ident_and_maybe_parsers` | Pass | Pass |
| **AST & Diagnostics** | `ast_tag_root_and_fold_ast_build_hierarchy` | Pass | Pass |
| **AST & Diagnostics** | `error_display_and_filename_propagation` | Pass | Pass |

*(For the 5 integration performance benchmarks in [`tests/perf_2000_lines.rs`](../tests/perf_2000_lines.rs), see [`report/2000-line-parsing-speed-report.md`](2000-line-parsing-speed-report.md).)*

---

## 4. How to Reproduce Locally

```bash
# Debug profile
cargo test --lib --verbose
cargo run --example simple_test

# Final Release profile
cargo test --release --lib --verbose
cargo run --release --example simple_test
```
