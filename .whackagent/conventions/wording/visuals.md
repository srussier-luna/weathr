# Visuals — rung 2 "Show"

## Rules

1. **Use what the project already has.** Detect the icon system and design-system components before proposing anything. Never add a dependency. If nothing fits, propose without implementing.
2. **Verify icon names exist.** Never invent an SF Symbol, Material Symbol or lucide name. If unsure, say so instead of guessing.
3. **Icon alone only when meaning is universal**: close, add, delete (trash), share, search, settings (gear), edit (pencil), back, more (ellipsis), favorite (heart/star), play/pause. Anything else → icon + short label, or label alone. A mystery icon is worse than a word.
4. **Native states before custom UI**: prefer platform components and states over inventing text.
5. **Accessibility label is mandatory** on every icon-only control, carrying the text it replaced.
6. **Structural changes stay in scope.** Replacing existing text with a visual, or adding a UI element, only when the task covers that screen; else say it in `NOTES:`.

## Detecting the icon system

| Stack | Where to look | Icon source |
|---|---|---|
| SwiftUI / UIKit | `Image(systemName:)`, `UIImage(systemName:)`, asset catalogs | SF Symbols |
| Compose / Android | `Icons.Default.*`, `Icons.Rounded.*`, `res/drawable` | Material Symbols |
| Web | `package.json`: `lucide-react`, `@heroicons/*`, `@phosphor-icons/*`, `react-icons`, `@mui/icons-material`, `@radix-ui/react-icons`, icon sprites | That library |

Also look for local design-system components: `Badge`, `EmptyState`, `Toast`, `Spinner`, `Skeleton`, `IconButton`, `Tooltip`.

## Native components and states

| Intent | SwiftUI | UIKit | Web | Compose |
|---|---|---|---|---|
| Destructive action | `Button(role: .destructive)` | `UIAlertAction(style: .destructive)`, `.destructive` menu attribute | danger variant, red | `MaterialTheme.colorScheme.error` |
| Empty state | `ContentUnavailableView` | `UIContentUnavailableConfiguration` | `EmptyState` component | Custom composable with icon + text |
| Search empty | `ContentUnavailableView.search` | same config `.search()` | — | — |
| Loading | `ProgressView`, `.redacted(reason: .placeholder)` | `UIActivityIndicatorView` | spinner / skeleton, `aria-busy="true"` | `CircularProgressIndicator` |
| Unavailable action | `.disabled(true)` | `isEnabled = false` | `disabled`, `aria-disabled` | `enabled = false` |
| Count | `.badge(3)` | `tabBarItem.badgeValue` | badge component | `BadgedBox` |
| Success feedback | `.sensoryFeedback(.success, trigger:)`, symbol swap with `.contentTransition(.symbolEffect(.replace))` | `UINotificationFeedbackGenerator` | icon swap + `aria-live="polite"` | `HapticFeedbackType` |
| Accessibility label | `.accessibilityLabel("Share")` or `Label("Share", systemImage:).labelStyle(.iconOnly)` | `accessibilityLabel` | `aria-label` | `contentDescription` |

In SwiftUI, prefer `Label("Share", systemImage: "square.and.arrow.up")` with `.labelStyle(.iconOnly)`: the text stays for VoiceOver and localization for free.

## Success without text

After a save, send, add, copy:
- the result is visible (item in list, new value shown) → nothing
- the result is not visible (copy to clipboard, background save) → brief icon swap to checkmark, haptic on mobile, polite live region on web
- a toast is justified only when the action has an undo, or its effect happens off-screen and matters
