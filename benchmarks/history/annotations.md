# Release annotations

Why the numbers in [REPORT.md](REPORT.md) moved. The data says *what* changed;
this file says *why*, cross-referenced with the [CHANGELOG](../../CHANGELOG.md).
Each entry lists the changes in that release most likely to affect benchmark
throughput, then records what was actually observed.

"Observed" reads REPORT.md's per-release deltas. Movements within the ±5%
reproducibility floor (see [README](README.md#comparability-caveats)) are treated
as noise unless they form a consistent pattern across many benchmarks.

## v0.7.1 — 2026-06-10

Net-flat versus v0.7.0 (median −0.2% across the shared benchmarks). Two movements
clear their spread and reproduce across two independent full runs, so they are
real:
- **GoogleMessage1 improved:** `decode` +12%, `merge` +14%, `decode_view` +10%
  (spreads ≤2.4%). The deeply-nested small message got faster on the owned and
  eager-view decode paths.
- **`media_frame/decode_view` regressed −11.6%** (spread ±0.9% — the most solid
  signal), and `log_record/decode_view` is marginally down (−4 to −6%, near the
  noise floor). The eager-view decode path lost ground on the bytes/map-heavy
  messages. This is the one regression worth investigating in v0.7.1.

Cautionary tale (why this history pins its build config): an earlier measurement
at cargo's *default* `bench` profile (`codegen-units=16, lto=off`, single sample)
showed a broad −3.3% regression and fingered `GoogleMessage1/decode_view` at
−17.5%. **All of that was build-layout noise.** At 16 codegen units, adding
unrelated code re-partitions functions and flips inline decisions; the v0.7.1
layout envelope across builds measured p50 5.8% / p90 15% / max 24%
(`layout_envelope.py`), and the broad deficit sat entirely inside it — the same
source measured −3.3% on one build and +0.3% on a fresh one. At the reproducible
profile this history now uses (`lto=true, codegen-units=1`, toolchain pinned,
median of 4 cores), `GoogleMessage1/decode_view` reverses from −17.5% to **+10%** —
it was never a regression. The lesson: a sub-~15% delta at the noisy profile means
nothing; pin the profile, hold the toolchain, and take the median of several runs
before attributing anything. (An earlier packed-varint over-reservation theory was
also unsupported here — the messages it would affect have no packed varint fields —
though it surfaced a separate, real allocation fix.)

## v0.7.0 — 2026-05-28

Likely perf-relevant changes:
- `reflect()` borrows the source instead of a bridge round-trip (reflection
  benchmarks are not in this set).
- Custom string/bytes types can take the raw payload and inline / take ownership
  zero-copy on the decode path.

Observed: essentially flat versus v0.6.0 (all core ops within ±5%). No
regression or improvement attributable to this release in this benchmark set.

## v0.6.0 — 2026-05-15

Likely perf-relevant changes:
- Map-field codegen emits ~40-50 inline lines per map field instead of a generic
  call path.
- Wire-type guard refactored across ~1,100 generated sites.
- Compile-time string literals remove a runtime allocation on some paths.

Observed: recovered the v0.5.0 encode regression — binary encode +12-13%
(ApiResponse, LogRecord, GoogleMessage1), view encode +11-16%, and view decode
improved (GoogleMessage1 +16%, ApiResponse +11%). The inlined map/wire-type
codegen is the most likely cause of the encode recovery.

## v0.5.0 — 2026-05-05

Likely perf-relevant changes:
- `unbox_oneof()` inlines non-recursive oneof variants, removing an allocation
  per construction.
- Zero-copy JSON serialization without `to_owned_message()`.
- `Any::clone()` becomes a refcount bump (not in this set).

Observed: two opposing effects. JSON encode jumped sharply (LogRecord +26%,
GoogleMessage1 +17%, MediaFrame +33%, AnalyticsEvent +10%) and GoogleMessage1
compute_size +8% — but the binary and view *encode* paths regressed (binary
encode ApiResponse −13%, LogRecord −9%, GoogleMessage1 −13%; view encode −6 to
−15%). v0.6.0 recovered the encode regression, so v0.5.0 looks like a JSON-encode
win that briefly cost the binary-encode path. Worth confirming which v0.5.0
change caused the binary-encode dip.

## v0.4.0 — 2026-04-27

Likely perf-relevant changes:
- `Bytes`-backed zero-copy decode: a field backed by a shared buffer is a
  refcount bump rather than an allocation + memcpy. Introduces the `MediaFrame`
  benchmark and the `*/encode_view`, `*/build_encode*` benchmarks.

Observed: GoogleMessage1 encode +9% (continuing v0.3.0's encode gains), but
AnalyticsEvent encode −10% and compute_size −8%, and ApiResponse compute_size
−6%. Mixed; the new view/build-encode benchmarks start their series here.

## v0.3.0 — 2026-04-01

Likely perf-relevant changes:
- The CHANGELOG [0.3.0] is dominated by features (extensions, text format, the
  `buffa-descriptor` crate) with no explicitly perf-targeted entry. The
  improvement below is therefore most plausibly a side effect of generated-code
  changes ("generated code emits `Self`", codegen restructuring) rather than a
  documented optimization. **Flagged to investigate** which codegen change moved
  it. All releases were built with the same toolchain (see below), so this is not
  a compiler effect.

Observed: the standout improvement release. GoogleMessage1 decode +16%, merge
+16%, encode +8%; ApiResponse decode +7%, encode +15%; LogRecord encode +12%.
The gains concentrate on GoogleMessage1 (a deeply nested message) and the encode
path generally, and they hold through later releases.

## v0.2.0 — 2026-03-16

Likely perf-relevant changes: none obviously perf-affecting.

Observed: flat versus v0.1.0 across every benchmark (all within ±3%), as
expected.

## v0.1.0 — 2026-03-07

Initial tracked release — the baseline for every series. The CHANGELOG's own
benchmark section reports binary encode 26-44% faster than prost 0.13 and JSON
decode 12-60% faster at this release.
