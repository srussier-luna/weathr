# Tones

Five defined tones. A tone changes **word choice and rhythm, never length**. Every tone still obeys the ladder and the blacklist.

Values for `base` in `wording.md` → **Project voice**: `neutral`, `sober`, `complice`, `premium`, `energetic`. A custom tone is written as free text in the **Voice** section with `base: custom`.

Situation-only tones (used by `situations.md`, not selectable as base): `serious`, `calm`.

## neutral — Neutre
Clear, factual, no personality. The safe default.
- Nouns and verbs, no adjectives. No humor. No exclamation marks.
- **Red line**: never cold to the point of blaming ("Invalid", "Denied").

| | en | fr |
|---|---|---|
| Empty | "No projects" | « Aucun projet » |
| Error | "Offline. Try again" | « Hors ligne. Réessayer » |
| Button | "Create project" | « Créer un projet » |
| Destructive | "Delete “Trip”? 12 photos will be deleted." | « Supprimer « Voyage » ? 12 photos seront supprimées. » |

## sober — Sobre
Minimal, calm, almost silent. Maximum removal, single words, lots of visuals.
- One word when possible. No sentences outside required text.
- **Red line**: never cryptic — a single word must still be unambiguous.

| | en | fr |
|---|---|---|
| Empty | "Empty" | « Vide » |
| Error | "Offline" + retry icon | « Hors ligne » + icône réessayer |
| Button | "New" | « Nouveau » |
| Destructive | "Delete “Trip”? 12 photos lost." | « Supprimer « Voyage » ? 12 photos perdues. » |

## complice — Complice
Warm, light, a touch of humor. Talks like a friend who respects your time.
- Informal phrasing, occasional wit on low-stakes moments (empty states, onboarding, success).
- **Red line**: never jokes on errors, loss, money, or anything destructive. Never cutesy ("Oopsie"). Humor never adds length.

| | en | fr |
|---|---|---|
| Empty | "Nothing here yet." | « Rien pour l'instant. » |
| Error | "Offline. Try again?" | « Pas de réseau. On réessaie ? » |
| Button | "Let's go" | « C'est parti » |
| Destructive | *(serious applies)* "Delete “Trip”? 12 photos will be gone." | « Supprimer « Voyage » ? 12 photos seront perdues. » |

## premium — Premium
Elegant, composed, carefully chosen words. Confidence without hype.
- Precise vocabulary, calm rhythm, slightly more evocative nouns. No slang, no exclamation marks, no emoji.
- **Red line**: never pompous or marketing-speak ("Elevate your experience"). Elegance is restraint.

| | en | fr |
|---|---|---|
| Empty | "Your first project awaits." | « Votre premier projet vous attend. » |
| Error | "Connection unavailable. Try again" | « Connexion indisponible. Réessayer » |
| Button | "Begin" | « Commencer » |
| Destructive | "Delete “Trip”? Its 12 photos will be permanently removed." | « Supprimer « Voyage » ? Ses 12 photos seront définitivement supprimées. » |

## energetic — Énergique
Direct, dynamic, motivating. Imperatives, momentum.
- Action verbs first, short punchy fragments. Exclamation marks allowed only on genuine achievements.
- **Red line**: never pushy or guilt-tripping ("Don't give up now!", fake urgency).

| | en | fr |
|---|---|---|
| Empty | "Start your first project" | « Lance ton premier projet » |
| Error | "Offline. Retry" | « Hors ligne. Relancer » |
| Button | "Start" | « Go » |
| Destructive | *(serious applies)* "Delete “Trip”? 12 photos will be deleted." | « Supprimer « Voyage » ? 12 photos seront supprimées. » |

## serious — situation only
Plain, precise, zero personality. Used for destructive actions whatever the base tone.
- Name the object, state the consequence, exact verb on the button.

## calm — situation only
Steady, blameless, solution-first. Used for errors.
- Never "you did", never humor, never alarm. What to do next.
