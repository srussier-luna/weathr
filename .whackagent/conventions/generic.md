# Conventions — Generic

> whackagent convention module · language-agnostic fallback, read by both verifiers (`conventions`, `correctness`). Replace or extend with the project's real conventions.

## Toggles

```yaml
public_doc: true          # document every public API surface
comment_discipline: true  # comments only explain non-obvious "why"
match_surrounding_style: true
```

## Review checklist

1. **Public documentation** (`public_doc`) — every public/exported symbol documented in language-idiomatic doc format.
2. **Comment discipline** (`comment_discipline`) — no comment restate code; only non-obvious *why*.
3. **Idiomatic** — code read like surrounding codebase: match naming, structure, patterns. Use language standard idioms, not patterns ported from other language.
4. **Match surrounding style** (`match_surrounding_style`) — naming, formatting, error handling, file layout follow existing files in same module.
5. **Errors** — handle and surface errors; no silent swallow.
6. **Testing** — tests live where project keep them, named clear.
7. **YAGNI · SOLID · DRY** — no speculative generality (YAGNI); one responsibility per unit, depend on abstractions, small composable pieces (SOLID); no duplicated logic — factor it, reuse what exists (DRY).

## Architecture (review category: architecture)

- **Group by feature/domain, not by type.** No global buckets holding whole codebase.
- **No flat tree.** Folders get sub-folders as grow; no pile files at root.
- **Coherent module boundaries**, dependencies point one direction, no cycles.
- Mirror source structure in tests.

## Build proof

Implementer must show project build and tests pass before report done.