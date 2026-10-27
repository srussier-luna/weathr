---
# whackagent config — written by /wa-setup, edit freely.
discussion_language: fr        # language Claude talk to you in
code_language: en              # identifiers, comments, logs, commits
ui_strings_language: fr        # user-facing strings
pr_language: ""                # PR titles + bodies. Empty → discussion_language
pr_title_prefix: false         # true → PR title start with commit prefix (`fix: …`): squash
                               # merges keep conventional commits on main. Not asked at setup.
pr_screenshots:                # PR screenshots of changed screens, at /wa-validate or /wa-close.
                               # Ignore verify.mode: taken even under autopilot / off.
  mode: auto                   # auto — UI changed → new screens shot, changed screens before/after · off
  width: 250                   # px in PR body, height auto (250 = iPhone)
primary_language: generic        # swift | typescript | generic | ...
project_kind: cli              # app | package | cli | server  (pick architecture module)

paths:                         # WHERE whackagent keep each file kind. Skills refer as
                               # {backlog} {tasks} {wiki} {reports} {conventions} — never literal path.
                               # Relative → from repo root; absolute OK (e.g. wiki in sibling repo).
                               # Team share → point at committed, browsable folder —
                               # `docs/wiki` read on GitHub, `.whackagent/wiki` no.
                               # Missing key → default below; old config still work.
  backlog: .whackagent/BACKLOG.md
  tasks: .whackagent/tasks
  wiki: .whackagent/wiki
  reports: .whackagent/reports          # run reports — usually local, gitignore-able
  conventions: .whackagent/conventions  # convention modules copied by /wa-setup
                               # `.whackagent/config.md` itself NOT configurable: carry these
                               # paths, must sit at known spot.
                               # Path moved after setup → move files too; no back-fill.

tasks:                         # WHERE tasks live. See wa-board → Task store.
  backend: github               # files  — one .md per task in {tasks}, order in {backlog}. Default.
                               # github — one issue per task on GitHub Project board. Task file
                               #   {tasks}/<n>-<slug>.md on task branch (truth); issue hold
                               #   title, summary, size, sprint, blockers. Plugin move cards
                               #   from pushes/merges, no GitHub Actions. Locks stop two people
                               #   grilling/coding same task. Every round commit + push;
                               #   one draft PR per task. {backlog} unused. Need `gh` with `project`
                               #   scope. See wa-board → Task store.
                               # plane  — same flow, one work item per task on Plane project (cloud
                               #   or self-hosted). Code, branches, PRs stay on GitHub. Need env
                               #   WA_PLANE_TOKEN (Plane API key).
  project: srussier-luna/2                  # github: <owner>/<number> of board · plane: project URL. Set by /wa-setup.
                               #   One board per repo, owned by whackagent.

review:
  when: on_validation          # WHEN wa-verifier run.
                               #   on_validation — once, at /wa-validate: your green light = feature
                               #     match spec, THAT fire review over whole diff (code + every
                               #     feedback round). Coding/feedback stay fast; nothing
                               #     reviewed while moving.
                               #   each_round — also after /wa-code and every /wa-feedback fix.
                               #     Catch drift earlier, cost verifier round each time. /wa-validate
                               #     still run final pass.
                               # Either way no task close unreviewed — /wa-close refuse task
                               # verifier never saw.
  inline_micro_fixes: true     # /wa-feedback may apply MICRO-fix itself vs spawn
                               # implementer (~50k tokens context for one-liner). Bound: ≤2 files,
                               # ≤~20 lines, no new file/type/folder, no layer or public-API change.
                               # Bigger or doubt → wa-implementer.
                               # false → every change via implementer, any size.
  autofix: true                # wa-code verify phase re-dispatch implementer until clean
  public_doc: false             # require doc on public API — flip false per company
  modules: [generic.md, wording.md, glossary.md]
                               # what single wa-verifier load before judging. Setup drop
                               # swiftui.md if no SwiftUI, swap architecture-app.md for
                               # architecture-package.md. Paths relative to {conventions}.
                               # Setup add apple.md when it exported Xcode SwiftUI
                               # guidance into {conventions}/apple/ (Xcode 27+) — index only,
                               # agents open one reference per touched topic.
                               # Setup add glossary.md (every project: one word per product
                               # concept, used in UI, tasks, reports, Claude speech) and,
                               # UI project, wording.md + {conventions}/wording/ (UI copy
                               # rules, project voice set at setup).
                               # ONE verifier, not one per lens: isolated agent cost ~50k tokens
                               # context before first line, every lens judge same diff vs
                               # same rulebook — split pay twice, duplicate findings to dedupe.
                               # Sweep style, elegance, structure, correctness in one pass,
                               # report which ran.

build:                         # how THIS project build — project beat plugin default
  command: "cargo build"       # e.g. "ign app", "make build", "./scripts/build.sh". Empty → XcodeBuildMCP
                               # for Apple targets, `swift build` for SwiftPM, project runner otherwise.
  test_command: "cargo fmt -- --check && cargo clippy -- -D warnings && cargo test"  # e.g. "make test". Empty → language default (`swift test`, …).

verify:                        # runtime check — implementer drive app it built
  mode: autopilot              # WHO exercise app after green build:
                               #   autopilot — agent drive it in /wa-autopilot only (nobody
                               #     testing); attended /wa-code + /wa-feedback stop at build + tests,
                               #     YOU validate by testing app. Default for app targets.
                               #   always — agent drive it every run, attended or not.
                               #   off — agent never drive; build + tests whole proof.
                               # Under `autopilot`/`off` implementer may STILL launch app when
                               # can't write feature without seeing it run (repro bug, judge
                               # layout, follow nav). That implementation, not proof: drive
                               # minimum needed, say so in NOTES.
  platform: ios                # ios | android | both  (ios via XcodeBuildMCP, else mobile-mcp)
  target: simulator            # simulator | emulator | device

commit:
  auto_commit_after_validation: false   # Claude may commit once YOU validate feature?
                               # (name kept for old configs: gate commit /wa-close make)
                               # github backend: ignored — every round commit + push (board need it)
                               # no author here, ever: commit use git config user.name/email
                               # of running machine. Never "Claude", never Co-Authored-By.

branch:
  per_task: true              # /wa-code work on own branch per task, not current
                               # github backend: always true — task file live on task branch
  prefix: "wa/"                # branch name: <prefix><key> → wa/add-apple-login (github: wa/42-add-apple-login)
  base: develop                # fork point: current | main | <branch name>
  sprint_prefix: "sprint/"     # task with `sprint:` branch off SPRINT branch, not base:
                               #   sprint/login-refacto   ← kebab of sprint "Login refacto",
                               #                            created from base: on first task
                               #   wa/add-apple-login     ← forked from it, merged back by /wa-close
                               #   (github backend: wa/42-add-apple-login)
                               # Task 3 see task 1 work — same screen, no blind conflict.
                               # Created by first who need it: /wa-code step 0 or /wa-autopilot
                               # wave setup. Empty string → no sprint branch, tasks use base:.
                               #   sprint/login-refacto-test ← LOCAL test branch: sprint + every
                               #                            delivered task, rebuilt + checked out after
                               #                            each delivery and end of /wa-autopilot.
                               #                            Launch project there, test whole sprint.
                               #                            Never pushed, never landed.
  checkout_next: true          # after /wa-close commit, hop to next task branch
                               # (only when per_task AND commit.auto_commit_after_validation)

close:                         # WHERE work land when /wa-close finish task. Sprint task
                               # always merge into sprint branch first — this block say fate
                               # of branch with nowhere left to go (standalone task,
                               # or sprint branch once last task close).
  strategy: nothing            # nothing — /wa-close stop after commit, branch untouched.
                               #   YOU open PR. Safest, default.
                               # pr     — push branch + open PR onto target: (`gh pr create`).
                               #   No confirm: /wa-close say branch → target + PR title, then do it.
                               # merge  — merge branch into target: locally, no push.
                               # github backend: ignored — task already have draft PR;
                               #   /wa-close rebase, push, mark ready. YOU merge.
  target: develop              # where pr/merge land. Ignored by `nothing`.
  delete_branch: auto          # auto   — delete only once code live elsewhere (merged into sprint
                               #   branch or target). `pr` and `nothing` keep it:
                               #   PR need branch, so do you.
                               # always — delete after close regardless. Ask first when
                               #   branch unmerged — throw work away.
                               # never  — /wa-close never delete branch.
                               # Slug worktree (autopilot leftover) removed with it.

autopilot:
  on_blocker: skip-and-log     # never invent; freeze task, move on
                               # autopilot ALWAYS branch per task, whatever branch.per_task say

yagni: strict
compress_wiki: true            # caveman-compress config + wiki pages, save re-read tokens
                               # (at /wa-setup + after every /wa-wiki update; .original backups removed)
---

# Project config

Free-form project notes every skill keep in mind.
Edit frontmatter above to change behavior.

## Core rule — stop and ask

Outside autopilot, anything unclear, ambiguous, or blocked beyond task spec: **stop and ask**. Never guess scope.

**Every question come with recommended answer** — always, no exception. One line pick, one line why, plus alternative if real one exist. No basis to choose? Recommend most reversible option, say it guess. Bare question without proposal never OK: answer must be confirm-or-correct, not homework.

Inside autopilot: never ask (nobody watching) — freeze task, log open question, move to next.

## Project notes

- Rust (edition 2024) TUI binary: `generic` module, no Rust-specific one. Proof = CI gate (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`).
- Fork of Veirt/weathr, customized with French village story. UI strings French, `tu`.- Versions: a release = merge `develop` → `main` (players install `main`), tagged `vX.Y.Z`. Sprints stay grouped by theme; the target version is a label on the issue:
  - `vX` (`v1`, `v2`, `v3`…) = any `X.y` release, e.g. `v2` = 2.0, 2.1…
  - `vX.Y` (`v1.6`…) = exactly that release.
- `/wa-task` grill: always ask which version the task targets (`vX` or `vX.Y`), then put that label on the issue (create it if missing). Every open task carries one.
- Never merge into `develop` a PR whose label targets a release not being prepared: `vX` waits for the first `X.y` release, `vX.Y` for `X.Y`.
