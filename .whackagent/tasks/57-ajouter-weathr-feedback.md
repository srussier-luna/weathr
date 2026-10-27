---
issue: 57
status: todo
wiki: [[updater]], [[architecture]]
note: "une commande weathr --feedback pour proposer aux utilisateurs de donner leur avis et de proposer des améliorations, il faudrait un endroit dans GitHub pour les répertorier. Peut-être une colonne retour utilisateur à gauche de todo ou tout autre outil GitHub qui nous permettrait de stocker et trier les items voulus et non voulus"
created: 2026-10-27
---

## Context / Decisions

> une commande weathr --feedback pour proposer aux utilisateurs de donner leur avis et de proposer des améliorations, il faudrait un endroit dans GitHub pour les répertorier. Peut-être une colonne retour utilisateur à gauche de todo…

- No blocker, no sprint. New CLI flag `--feedback` (`src/main.rs`, clap, like `--update`): runs before the TUI, exits after.
- **Where feedback lands**: GitHub issues on `srussier-luna/weathr`, labels `retour-utilisateur` + `wa-ignore`.
  - `wa-ignore` keeps them off the whackagent board until triaged.
  - Keep one → `/wa-task <n>` adopts it as a draft. Drop one → close as « not planned ».
  - Triage view = issues filtered by label `retour-utilisateur`.
  - No board column: whackagent owns the board's columns.
- **Flow in the terminal** (French, `tu`, `[weathr] …` system voice, `{conventions}/wording.md`):
  1. Type: `Avis`, `Idée` or `Souci` (pick 1/2/3).
  2. Message, several lines; an empty line ends it. Empty message → cancel, nothing sent.
  3. Preview (title + body), then `Envoyer ? [O/n]`. `n` → nothing sent.
- **Issue content**:
  - Title: `<Type> : <first line of the message>`, cut to ~60 chars.
  - Body: the message, then a footer with weathr version (`CARGO_PKG_VERSION`), OS and date. Nothing else: no location, no progress, no config.
- **Sending**: `gh issue create --repo srussier-luna/weathr --title … --body-file … --label retour-utilisateur --label wa-ignore`.
  - Success → `[weathr] Merci ! Ton retour est noté : <issue URL>`.
  - `gh` missing, not logged in, or create fails → nothing lost: print a prefilled new-issue URL (title + body + labels in the query string) to open in the browser.
- New module by feature: `src/feedback.rs`. Prompt parsing, title/body building and URL building pure and unit-tested; `gh` call thin.
- **Before release (not code)**: the repo is private, so only members can open issues. Make `srussier-luna/weathr` public, and create labels `retour-utilisateur` and `wa-ignore` (`gh issue create --label` fails on a missing label).
- Out of scope: in-TUI feedback, ratings, anonymous channel without GitHub account, Discussions.

## Acceptance criteria

- [ ] `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` all green.
- [ ] `weathr --help` lists `--feedback`.
- [ ] `weathr --feedback`, pick `2`, type two lines then an empty line: preview shows `Idée : <first line>` and the body with version, OS, date.
- [ ] Answer `O`, `gh` logged in: an issue is created with labels `retour-utilisateur` and `wa-ignore`, its URL printed.
- [ ] Answer `n`: nothing created, `weathr` exits.
- [ ] Empty message: cancelled, nothing created.
- [ ] `gh` not on `PATH`: prefilled issue URL printed, title and body in it.
- [ ] The new issue doesn't appear as a draft on the whackagent board.
- [ ] Title cut, body footer and prefilled URL encoding tested (test).

## Implementation

## Review

## Verification

## Feedback
