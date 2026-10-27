# Wording — UI copy

> whackagent convention module · UI projects only. `wa-implementer` writes every user-facing string against it, `wa-verifier` judges them. Edit **Project voice** per project. Reference pages sit in `wording/`, next to this file — plugin text, refreshed by `/wa-update`, never edited by hand. Same rules as the `wa-wording` skill, which covers UI written outside a task.

Great interface copy is mostly **absent**. Every string earns its place. UI shows; text only says what UI cannot.

Applies to every user-facing string: button labels, titles, subtitles, empty states, errors, alerts, confirmations, toasts, loading, onboarding, paywalls, permission prompts, accessibility labels, localization catalogs.

## Project voice

Written by `/wa-setup`, yours to edit.

```yaml
languages: [fr]          # source first — mirrors ui_strings_language in config
person: tu               # tu | vous | impersonal (per-language when needed: {fr: tu, de: du})
base: complice           # neutral | sober | complice | premium | energetic | custom
audience: tech           # general | tech | young | pro
situations:              # override tones only; guardrails are fixed (wording/situations.md)
  destructive: serious
  error: calm
  payment: neutral
  permission: base
  empty-state: base
  success: base
  onboarding: base
  legal: neutral
```

**Voice** — two or three sentences on how the app talks. Allowed / forbidden: emoji, exclamation marks, humor.

weathr tutoie, chaleureux et un peu malicieux : le village parle comme des voisins qui se connaissent. Messages système (`[weathr] …`) courts et factuels, humour réservé aux lettres, cartes postales et détails du village. Pas d'emoji ; points d'exclamation rares, seulement dans les lettres des habitants.

Notes de version : tout ce qui est sur GitHub (PR de version `develop` → `main`, Release, tag) est une vraie note de version, factuelle — sections Nouveautés / Changements / Mise à jour, une ligne par changement avec son numéro de PR. Les jolies phrases rigolotes, uniquement dans le jeu (`NOUVEAUTES.md`, boîte Nouveautés).

Tickets de saison (sprints « Saison N ») : titre et résumé sans spoiler de l'histoire — ni nouveau lieu, ni personnage, ni Habitant, ni événement nommé ; seuls les mots déjà connus en saison 1 (Lettres, Habitants, Voisin, scène). Les détails vont dans un bloc `<details><summary>Spoiler</summary>` replié, sous le résumé.

<!-- to review -->

**Validated examples** — strings the user rewrote, `before → after · situation`. Max 12, 2 per situation. Read them before `wording/tones.md`: this project's voice beats the generic tone.

<!-- - "Your trip has been saved!" → (removed, checkmark) · success -->
- « Ta première lettre est arrivée. » → « Je viens de te déposer ton courrier. » · onboarding
- « Lis tout. Certains détails comptent plus qu'ils n'en ont l'air. » → « Lis bien tout. Certains détails comptent plus qu'ils n'en ont l'air ;) » · onboarding

## The ladder — every string

Go down in order, **stop at first rung that works**. Most strings stop at 1 or 2. Details + before/after: `wording/ladder.md`.

1. **Remove** — user understands without it? Title repeating button, subtitle repeating title, text explaining what UI shows → delete.
2. **Show** — a visual can carry it? Icon, button state (disabled, spinner, checkmark), badge, color, haptic, native empty-state component → replace. Rules: `wording/visuals.md`.
3. **Shorten** — text needed → minimum words. Buttons are verbs. No sentence where a word works.
4. **Tune** — apply `base` tone, or tone of detected situation.

Then reject anything in `wording/blacklist.md`. Worst offenders:

- narrating what app did: "Your changes have been saved successfully"
- explaining how UI works: "Tap the button below to…"
- filler politeness, vague reassurance, dev jargon, repeating what user just did

**Text required** — never remove, never shorten past meaning:

- irreversible destructive action: name what is lost
- error: say what to do
- legal, consent, pricing

## Words

- **Glossary wins.** `glossary.md` (same folder): **Concepts** term in each UI language, never a `Never` word. **Screens**, **Parts**, **Flows** are team names, not UI copy.
- New concept with no glossary entry → pick the word user says out loud, flag it in `NOTES:` so it gets added.
- **Write native, never translate.** Write directly in target language. Words target user says out loud, platform vocabulary, common English loanwords over invented or stiff equivalents. Rules per language: `wording/native.md`.
- French: before `? ! : ;` and inside `« »` the space is **non-breaking** — write it as escape (` ` in JSON/JS, `\u{202F}` in Swift), never a typed space.

## Read on demand, never whole

| You touch | Open |
|---|---|
| any string you are unsure about | `wording/ladder.md` |
| replacing text with icon / state / native component | `wording/visuals.md` |
| delete, error, payment, permission, empty state, success, loading, onboarding, legal | `wording/situations.md` — that situation only |
| tone other than `neutral`, or doubt on tone | `wording/tones.md` — that tone only |
| any language other than English | `wording/native.md` — that language only |
| localization catalog, keys, several languages | `wording/i18n.md` |
| final check on copy you wrote | `wording/blacklist.md` |

Diff with no user-facing string → open nothing.

## Scope

- Strings the task creates or changes. Never rewrite unrelated existing copy.
- Replacing text with icon: **always keep text as accessibility label** (`accessibilityLabel`, `aria-label`, `contentDescription`).
- Never add a dependency, never add a language that doesn't exist yet, never introduce an i18n system the project lacks.
- Icon names: verify they exist (SF Symbols, Material, project icon library). Never invent one.

## Reviewing

Lens:

- situation guardrail broken (destructive without object + consequence, error without next step, price or renewal unclear, consent buttons unequal) → `correctness`
- glossary term replaced by synonym, hardcoded string bypassing project catalog, missing accessibility label on icon-only control → `correctness`
- anything else (blacklist pattern, string that should be removed or shown, too long, off-tone, translated-sounding) → `style`

Finding names the string and the rewrite, cites page:

```
Views/TripRow.swift:42: 🟡 minor: [style] "Your trip has been saved successfully!" narrates app. Remove, checkmark on button — wording/blacklist.md.
Views/TripDetail.swift:88: 🟠 major: [correctness] Delete alert "Are you sure?" names nothing. "Delete “Lisbon”?" + "12 photos will be deleted." — wording/situations.md.
```

Existing copy the diff doesn't touch → not a finding.
