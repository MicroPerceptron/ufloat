| Type | Candidate Layout | vs. IEEE 754 analogue                     |
| ---- | ---------------- | ----------------------------------------- |
| UF8  | E4M4 or E5M3     | vs. E4M3 (ML formats like FP8)            |
| UF16 | E5M11 or E6M10   | vs. E5M10 (float16)                       |
| UF32 | E8M24            | vs. E8M23 (float32), gains 1 mantissa bit |

## Benchmarks

The benchmark suite uses nightly's built-in `test` harness and covers conversions,
arithmetic, and raw-bit ordering for `Uf8`, `Uf16`, and `Uf32`.

```sh
cargo bench --bench arithmetic
cargo bench --features f16 --bench arithmetic
cargo bench --all-features --bench arithmetic
```
