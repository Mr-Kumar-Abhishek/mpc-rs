# 2,000-Line Parsing Speed Test Report (`mpc-rs`)

## 1. Executive Summary & Claim Verdict

- **Claim Tested:** *"A file with 2000 lines of input would need more than a second to be parsed."*
- **Verdict:** **Disproved.** Across four representative grammars in [`tests/perf_2000_lines.rs`](../tests/perf_2000_lines.rs), `mpc-rs` parses 2,000 lines in **10.27–17.31 ms** under release optimization (`~58x–97x` faster than 1 second) and **28.09–52.72 ms** even in unoptimized debug builds (`~19x–35x` faster than 1 second).
- **Scaling:** Increasing input size `4x` (from 500 to 2,000 lines) yields a **`3.16x`** latency ratio in debug mode and **`3.33x`** in release mode, confirming linear `O(n)` behavior without quadratic backtracking degradation.

---

## 2. Test Environment & Execution Metadata

| Property | Value |
|---|---|
| **Repository / Commit** | `Mr-Kumar-Abhishek/mpc-rs` (`7883a8d`) |
| **Test Suite** | [`tests/perf_2000_lines.rs`](../tests/perf_2000_lines.rs) + unit suite in [`src/lib.rs`](../src/lib.rs) |
| **CI Workflow / Run ID** | [`.github/workflows/rust.yml`](../.github/workflows/rust.yml) — Run `#44797208309` (`success`) |
| **Runner OS** | `ubuntu-latest` (GitHub Actions) |
| **Execution Profiles** | Debug (`cargo test --verbose -- --nocapture`) and Release (`cargo test --release --verbose -- --nocapture`) |

---

## 3. Benchmark Results (2,000 Lines per Test Case)

### 3.1 Debug vs. Release Parsing Latency & Throughput

| Test Case | Input Size | Debug Latency | Debug Throughput | Release Latency | Release Throughput | `< 1s` Gate |
|---|---:|---:|---:|---:|---:|:---:|
| `parse_2000_lines_plain_text` | 70,000 B | `28.09 ms` | `71,203 lines/s` (`2.38 MB/s`) | `10.27 ms` | `194,827 lines/s` (`6.50 MB/s`) | Pass |
| `parse_2000_lines_key_value_config` | 36,500 B | `35.66 ms` | `56,078 lines/s` (`0.98 MB/s`) | `12.00 ms` | `166,654 lines/s` (`2.90 MB/s`) | Pass |
| `parse_2000_lines_ast_expressions` | 42,000 B | `41.48 ms` | `48,221 lines/s` (`0.97 MB/s`) | `13.63 ms` | `146,731 lines/s` (`2.94 MB/s`) | Pass |
| `parse_2000_lines_csv_rows` | 64,000 B | `52.72 ms` | `37,933 lines/s` (`1.16 MB/s`) | `17.31 ms` | `115,526 lines/s` (`3.53 MB/s`) | Pass |

### 3.2 Workload Descriptions

1. **`parse_2000_lines_plain_text`**: Scans 2,000 lines (`line 0000: hello world from mpc-rs\n`) using `mpc_many(mpc_noneof("\n"))` and `mpc_newline()`, verifying `input.state.row == 2000` and full byte consumption.
2. **`parse_2000_lines_key_value_config`**: Alternates across integers (`12345`), hex (`0x1aF`), floats (`3.14f`), and string literals (`"val_0003"`), exercising `mpc_ident`, `mpc_or`, `mpc_float`, `mpc_number`, and `mpc_string_lit`.
3. **`parse_2000_lines_ast_expressions`**: Parses 2,000 arithmetic assignments (`var_0000 = 100 + 200\n`) into a full recursive `MpcAst` (`>|program` root with `2,000` `assign` child nodes) using `mpca_root`, `mpca_tag`, and `mpcf_fold_ast`.
4. **`parse_2000_lines_csv_rows`**: Parses 2,000 multi-type CSV rows (`0000,item_0000,3.14,"desc_0000"\n`) via `mpc_sepby1`.

---

## 4. Linear Scaling Check (500 vs. 2,000 Lines)

Test [`parse_2000_lines_linear_scaling_check`](../tests/perf_2000_lines.rs) averages 3 runs of the key-value config parser at 500 lines vs. 2,000 lines (`4x` input growth):

| Profile | 500 Lines (Avg) | 2,000 Lines (Avg) | Measured Ratio (`4x` Input) | Assertion (`< 12.0x`) |
|---|---:|---:|---:|:---:|
| **Debug** | `8.75 ms` | `27.64 ms` | `3.16x` | Pass |
| **Release** | `2.71 ms` | `9.04 ms` | `3.33x` | Pass |

---

## 5. Full Test Suite Summary (`Run #44797208309`)

| Suite / Profile | Passed | Failed | Ignored | Reported Suite Time |
|---|---:|---:|---:|---:|
| Unit tests (`src/lib.rs`) — Debug | 21 | 0 | 0 | `0.00s` |
| Performance suite (`tests/perf_2000_lines.rs`) — Debug | 5 | 0 | 0 | `0.11s` |
| Unit tests (`src/lib.rs`) — Release | 21 | 0 | 0 | `0.00s` |
| Performance suite (`tests/perf_2000_lines.rs`) — Release | 5 | 0 | 0 | `0.04s` |

---

## 6. How to Reproduce Locally

Run the performance integration suite in both profiles:

```bash
# Debug profile (unoptimized + debuginfo)
cargo test --test perf_2000_lines --verbose -- --nocapture

# Final Release profile (optimized, enforces < 1s assertions)
cargo test --release --test perf_2000_lines --verbose -- --nocapture
```
