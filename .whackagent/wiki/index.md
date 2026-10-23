# Wiki — Index

Living knowledge base for project. Pages link to each other and to tasks via `[[wikilink]]` syntax (Obsidian-compatible). Keep current via `/wa-wiki`.

## Pages

- [glossary](../conventions/glossary.md) — one word per thing: concepts, screens, parts, flows
- [[architecture]] — overall structure: startup, main loop, weather → scene flow, module map, files on disk
- [[scene]] — static landscape: house, mountains, river, tree, mailbox, layout invariants
- [[animation]] — moving systems, lore/unlocks, seasons, Fêtes, Facteur, demo
- [[weather]] — Open-Meteo, units, location, caches, rain alert, story fetches
- [[render]] — diffing terminal renderer, colours, overlay boxes, views, HUD
- [[mail]] — Lettre timeline, Carte postale, Gazette, Énigme, Bouteille, Badges, progress save
- [[updater]] — git-based self-update, CLI flags

## How to read

- Page describes domain, decision, or subsystem — *why* and *shape*, not code copy.
- Tasks reference wiki pages in `wiki:` frontmatter; wiki pages can link back to tasks.
- Code = structural truth; wiki = human narrative on top — explains what source can't tell.