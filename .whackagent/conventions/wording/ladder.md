# The ladder

Go down in order. Stop at the first rung that works. Most strings should stop at rung 1 or 2.

## 1. Remove

Ask: *if this string disappeared, would the user miss anything?*

Remove when the string:
- repeats another element (title = button label, subtitle = title, alert message = alert title)
- describes what is visible (a list header "Your projects" above an obvious list of projects in a screen titled "Projects")
- explains an interaction the UI already affords (chevron, tappable card, swipe action, text field with a cursor)
- confirms something the user can already see happened (the item appeared in the list)
- labels a field whose placeholder or context is unambiguous — keep the label if the field could be misread once filled

| Before | After |
|---|---|
| Title "Delete project" + message "Do you want to delete this project?" + button "Delete" | Title "Delete “Trip”?" + message with consequence + button "Delete" |
| Screen "Projets" + section header "Vos projets" | Screen "Projets", no header |
| Card with chevron + caption "Tap to see details" | Card with chevron |
| Row added to list + toast "Item added" | Row added (animate insertion) |

## 2. Show

Ask: *can something visual say this without words?* See `visuals.md` for rules and per-platform components.

| Before | After |
|---|---|
| Toast "Saved successfully" | Button turns into a checkmark for ~1 s (+ success haptic on iOS) |
| Label "Loading your data, please wait…" | Spinner / skeleton |
| Text "3 unread messages" next to Inbox | Badge `3` |
| Button "Share" in a toolbar | `square.and.arrow.up` icon, a11y label "Share" |
| "No results found for your search" | Native empty state: magnifying glass + "No results" |
| "This field is required" under an untouched field | Nothing until submit; then red outline + short message |

## 3. Shorten

Text is needed. Use the fewest words that keep the meaning.

- Buttons: a verb, the object only if ambiguous. "Save", "Delete", "Add photo". Never "OK" / "Yes" / "Submit" when a precise verb exists.
- Titles: noun or question. No trailing period.
- Messages: one sentence max, usually a fragment.
- Cut: "successfully", "please", "currently", "simply", "just", "in order to", "you can", "be able to".
- Numbers over words: "3 photos", not "three photos".
- Front-load the key word: "Wi-Fi off" beats "You are currently not connected to Wi-Fi".

| Before | After |
|---|---|
| "Enregistrer les modifications" | "Enregistrer" |
| "You don't have any notifications at the moment" | "No notifications" |
| "Click here to create your first project" | "Create project" |
| "Veuillez saisir votre adresse e-mail" | "E-mail" (label) |

## 4. Tune

Apply the base tone from `wording.md` → **Project voice**, overridden by the situation (`situations.md`). Tone changes word choice and rhythm, **never length**: a playful string is not a longer string.

| Neutral | Complice |
|---|---|
| "No projects" | "Nothing here yet." |
| "Connection lost. Retry" | "Offline. Try again?" |

## Required text — never remove

- **Irreversible destructive action**: name the object and what is lost. "12 photos will be deleted."
- **Error**: what to do next. Not what happened technically.
- **Legal, consent, pricing, renewal**: clarity over brevity.
- **Icon-only controls**: text stays as accessibility label.
