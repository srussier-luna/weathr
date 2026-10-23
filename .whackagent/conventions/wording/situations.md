# Situations

Detect the situation from the code, apply its tone and its guardrails. No need to ask.

- **Tone** can be overridden per project in `wording.md` → **Project voice** → `situations:`. `base` means "inherit the base tone".
- **Guardrails are fixed.** Config cannot disable them.

| Situation | Detection signals | Default tone |
|---|---|---|
| `destructive` | `role: .destructive`, `.destructive` style, delete/remove/reset/erase/revoke/unsubscribe/leave, `variant="danger"`/`"destructive"`, red buttons | `serious` |
| `error` | `catch`, `Result.failure`, `.alert(isPresented:error:)`, error toasts/banners, `onError`, validation messages | `calm` |
| `payment` | paywall, checkout, subscription, StoreKit/`Product`, Stripe, price labels, trial | `neutral` |
| `permission` | camera, photos, notifications, location, contacts, tracking prompts, `NS*UsageDescription` | `base`, softened |
| `empty-state` | empty list/collection branch, `ContentUnavailableView`, `isEmpty` rendering | `base` |
| `success` | after save/send/create/copy completion handlers | `base` |
| `loading` | `ProgressView`, spinners, skeletons, `isLoading` | — |
| `onboarding` | first launch, welcome, tutorial, `hasSeenOnboarding` | `base`, most expressive |
| `legal` | terms, privacy, consent, GDPR/cookies, age gate | `neutral` |

## Guardrails

### destructive
- Title names the object: "Delete “Trip”?" — never "Are you sure?".
- Message states what is lost, concretely: "12 photos will be deleted." Omit the message only if nothing beyond the object is lost.
- Confirm button = the exact verb ("Delete", "Remove", "Leave"). Never "OK", "Yes", "Confirm".
- Cancel stays "Cancel" (platform word).
- No humor, no emoji, no softening ("Just so you know…").
- If the action is undoable, prefer no confirmation + an Undo affordance over a dialog.

### error
- Say what to do, not what happened internally. "Offline. Try again" not "Network request failed".
- Never blame: no "You entered", "Invalid", "Wrong".
- Validation: state the expectation. "8 characters minimum", not "Password invalid".
- Show errors at the point of failure (inline under field) before resorting to alerts.
- No codes unless support needs them — then at the end, secondary style.

### payment
- Price, period and renewal always explicit: "€4.99/month, renews automatically".
- Trial: what happens when it ends, and when.
- No fake urgency, no guilt on decline ("No thanks, I don't like saving money").
- Button says what happens: "Start free trial", "Subscribe for €4.99/month".

### permission
- One sentence: the benefit to the user, in their terms. "To scan documents" not "App needs camera access".
- Ask in context (at the moment of use), with a pre-prompt only if the benefit isn't obvious.
- `NS*UsageDescription` strings follow the same rules and the same language rules.

### empty-state
- Show first: icon or illustration from the project.
- Max one line of text + one action. "No projects" + "Create project".
- Search empty: "No results" — and the query if helpful. No tips paragraph.
- Never explain the feature at length here.

### success
- Default: **no text**. The result is visible, or use checkmark/haptic (see `visuals.md`).
- Toast only with Undo, or when the effect is off-screen and matters.

### loading
- No text, unless it lasts more than ~2 s or has real steps ("Uploading 3 of 12").
- Never "Please wait", never fake steps.

### onboarding
- One screen = one idea. Title + at most one line.
- Show the product, not a description of it.
- Skip is always available and labeled plainly.

### legal
- Clarity beats brevity. Do not shorten to the point of changing legal meaning.
- Do not rewrite legal text provided by a lawyer or a policy source — only its surrounding UI (buttons, links, headings).
- Consent buttons are symmetrical in weight: "Accept" / "Decline".
