# Project decisions

## Parallel search (2026-10-23)

Use scoped, named threads with a shared atomic cancellation flag. Each worker owns
its RNG through generation calls and borrows the fixed first phrase. A bounded
channel holds at most one result per worker. Stop peers after success or a
generation error or caught worker panic; join every worker before returning, including when
thread creation fails. Relaxed ordering suffices because the flag only signals
cancellation; results are synchronized through the channel and joins.

Preserve the existing composite entropy source and CPU-count default; changing
cryptographic generation is outside this threading refinement.

Old private-key derivation tests referenced APIs removed before this change.
Remove those obsolete tests, update `generate()` tests to `generate24()`, and
exercise phrase compatibility, cancellation, and invalid worker counts instead.
