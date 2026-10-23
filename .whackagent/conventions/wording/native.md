# Native, not translated

## Universal rules

1. **Write directly in the target language.** Never draft in English and translate. Translation leaks English word choice *and* English sentence structure.
2. **Word choice, in order:**
   1. The project glossary (`glossary.md`).
   2. What the target user says out loud. Adjust with `audience`: tech or young audiences accept more English loanwords than general or senior audiences.
   3. The platform's own vocabulary (the OS the app runs on).
   4. Never an invented, rare or dictionary-only word. If the local word sounds like a machine translation, keep the common English loanword.
3. **Idioms don't translate.** Rewrite the intent, not the words.
4. **Test:** read it aloud as a native speaker who uses the app daily. If they would never say it, rewrite.

## French (fr)

### Loanwords vs French — decide by usage

| Keep the loanword (what people say) | Use French (what people say) | Never (sounds translated or invented) |
|---|---|---|
| liker, like | tableau de bord | mettre à niveau (upgrade) |
| story, feed | se connecter | infolettre (outside Québec) |
| streak | brouillon | mot-dièse (hashtag) |
| swiper | partager | courriel (outside Québec) |
| hashtag | abonnement | flux (for a social feed) |
| e-mail | télécharger | clavardage |
| podcast | notifications | baladodiffusion |
| check-in | paramètres / réglages (see platform) | sélectionneur |

Audience shifts this: a pro B2B app may prefer "fil d'actualité" over "feed". The glossary settles disputes.

### Platform vocabulary

| Concept | Apple (iOS/macOS) | Android / Google | Web (neutral) |
|---|---|---|---|
| Settings | Réglages | Paramètres | Paramètres |
| Share | Partager | Partager | Partager |
| Trash | Corbeille | Corbeille | Corbeille |
| Sign in | Se connecter | Se connecter | Se connecter |
| Cancel | Annuler | Annuler | Annuler |
| Done | OK | OK | Terminé |

### Literal translation traps

| English | ✗ Literal | ✓ Native |
|---|---|---|
| Upgrade | Mettre à niveau | Passer à Pro / Passer à Premium |
| You're all set | Vous êtes prêt | C'est bon / C'est prêt |
| Get started | Obtenir commencé / Démarrer maintenant | C'est parti / Commencer |
| Something went wrong | Quelque chose s'est mal passé | (say what to do) Réessayer |
| Learn more | Apprendre plus | En savoir plus |
| Sign up | Signer | S'inscrire / Créer un compte |
| Let's go | Allons-y | C'est parti |
| No results found | Aucun résultat trouvé | Aucun résultat |
| Are you sure? | Êtes-vous sûr ? | (name the action) Supprimer « Voyage » ? |
| Keep me signed in | Garde-moi connecté | Rester connecté |
| Welcome back! | Bienvenue de retour ! | Content de te revoir / Re-bonjour |
| Tap to retry | Tapez pour réessayer | Réessayer |
| Add to cart | Ajouter au panier | Ajouter au panier ✓ (already native) |

Structural calques to avoid:
- Passive voice everywhere ("Votre compte a été créé") → active or nothing.
- Possessive overuse ("Vos paramètres", "Votre profil") → "Paramètres", "Profil".
- Gerund-style labels ("Chargement de vos données…") → spinner, or "Chargement…" at most.

### Typography

- **Sentence case only.** "Mes projets", never "Mes Projets". French has no Title Case.
- **Non-breaking space before `: ; ! ?`** and inside guillemets `« … »`. Use a narrow no-break space (U+202F) when the platform renders it; otherwise U+00A0. Never a regular space (line breaks before "?").
  A typed space is always a regular U+0020 — **write the escape explicitly** so it survives:

  | File | Escape | Example |
  |---|---|---|
  | JSON, `.xcstrings`, JS/TS | ` ` | `"Supprimer ?"`, `"« {name} »"` |
  | Swift literal | `\u{202F}` | `"Supprimer\u{202F}?"` |
  | `.strings` | `\U202F` | `"Supprimer\U202F?"` |
  | Android XML | ` ` | `Supprimer ?` |
  | Kotlin | ` ` | `"Supprimer ?"` |
  | Raw HTML markup only | `&#x202F;` | `<p>Supprimer&#x202F;?</p>` |

  i18n files for web apps (i18next, react-intl, vue-i18n JSON) are **JSON, not HTML**: use ` `. An entity there renders literally as `&#8239;`.

  In a chat answer with no file, write the escape too, or state that the space is U+202F.
- **Guillemets** `« »` for quotes, not `"…"`.
- **Typographic apostrophe** `’` if the project already uses it; stay consistent.
- **Accented capitals**: "État", "À venir", "Écran".
- Numbers: `1 234,56 €` (space thousands separator, comma decimal, € after with no-break space). Prefer the platform formatter over hardcoding.
- Person: follow `person` in config. Never mix *tu* and *vous* in one app.

## English (en)

### Rules

- **Case**: follow the platform. Apple HIG → Title Case for buttons, menu items, alert titles; sentence case for body. Material and most web design systems → sentence case everywhere. Follow the project's existing convention if consistent.
- **Contractions** in casual tones ("Can't", "You're"); fewer in neutral/premium.
- **No Latinate words** when a short one exists: "use" not "utilize", "start" not "commence", "help" not "assist", "buy" not "purchase" (unless legal).
- **Straight vs curly quotes**: follow the project; prefer curly `’ “ ”` in shipped copy.
- **Serial comma**: follow the project; pick one.

### Traps

| ✗ | ✓ |
|---|---|
| "Please be patient" | Spinner |
| "Oops!" | Say what to do |
| "Successfully deleted" | Nothing, or "Deleted" + Undo |
| "Utilize your account" | "Use your account" |
| "In order to continue, sign in" | "Sign in to continue" |

## Adding a language

Create a new `## <Language> (<code>)` section with: loanword table, platform vocabulary, literal traps from English, typography.
