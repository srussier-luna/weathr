# Blacklist

Reject these patterns at every rung. They are the typical marks of AI-written UI copy.

## Narrating what the app did

The UI state already shows it. Show, don't tell.

| ✗ | ✓ |
|---|---|
| "Your changes have been saved successfully!" | Checkmark on the button, or nothing |
| "Le fichier a été importé avec succès." | The file appears in the list |
| "We've updated your preferences." | Toggle reflects the new state |
| "Project created! You can now start adding tasks." | Open the new project |

## Explaining how the UI works

If the UI needs a manual, fix the UI.

| ✗ | ✓ |
|---|---|
| "Tap the + button below to add an item." | "+" button |
| "Glissez vers la gauche pour supprimer." | Swipe action with trash icon |
| "Use the toggle to enable notifications." | Toggle labeled "Notifications" |
| "Select an option from the list below:" | The list |

## Describing behavior or internals

| ✗ | ✓ |
|---|---|
| "This will automatically sync across all your devices." | Nothing, or "Synced" state indicator |
| "Your data is stored locally and encrypted." (in a settings row) | Nothing — belongs in a privacy page if anywhere |
| "The list refreshes every 5 minutes." | Nothing |

## Filler politeness

| ✗ | ✓ |
|---|---|
| "Please enter your password" | "Password" |
| "Veuillez patienter…" | Spinner |
| "Merci de sélectionner une date" | "Date" |
| "Thank you for your patience" | Nothing |

## Vague reassurance and fake emotion

| ✗ | ✓ |
|---|---|
| "Don't worry, your data is safe." | Say concretely what happens, or nothing |
| "Oops! Something went wrong 😕" | "Couldn't load. Try again" |
| "Great job! 🎉" on routine actions | Nothing |

## Repeating the user's own action

| ✗ | ✓ |
|---|---|
| "You have selected 3 items." | "3 selected" in the toolbar, or checkmarks |
| "Vous avez modifié votre nom." | New name displayed |

## Dev jargon

| ✗ | ✓ |
|---|---|
| "Error 500: Internal Server Error" | "Server unavailable. Try again later" |
| "null", "undefined", "NaN" | Placeholder dash or hide the element |
| "Sync failed (code -1009)" | "Offline" |
| "Invalid input" | Say what is expected: "8 characters minimum" |

## Weak buttons

| ✗ | ✓ |
|---|---|
| "OK" / "Yes" on a destructive confirm | "Delete" |
| "Submit" | The actual action: "Send", "Book", "Pay €12" |
| "Click here" | The destination or action |
| "Continue" when something specific happens | "Create account" |

## Punctuation and formatting tics

- Exclamation marks on routine actions.
- Ellipsis "…" except on actions that open a further step (menu items, macOS convention) or real ongoing progress.
- Periods at the end of buttons, titles, labels.
- Emoji, unless the glossary or voice explicitly allows them.
- Title Case in languages that don't use it (see `native.md`).
