# 📚 Chapter 6 — The Grand Library

> *"A single scroll holds a spell. A library holds a thousand. And that library must be organised — with sections, catalogues, and testing chambers. Your code deserves the same."*

Your game has outgrown a single file. This chapter is about everything *around* the code: how Rust splits it into modules and crates, how Cargo ties crates together in a workspace (like the one this course lives in), and how you prove and explain that it works — with unit tests, integration tests, doc tests and documentation. It ends with feature flags: compiling different editions of the same library.

Coming from Go, think *packages and `go test`* — but with privacy per module instead of per capitalisation, tests living next to the code, and examples in docs that are actually executed. Coming from Python, think *packages and pytest* — but resolved at compile time, so a wrong import is a build error, not a runtime surprise.

| Quest | Topic |
|---|---|
| 6.0 | [Modules — The Shelves of the Library](00_modules.md) |
| 6.1 | [Crates & Packages — The Bound Tomes](01_crates_packages.md) |
| 6.2 | [Workspaces — The Citadel of Many Towers](02_workspaces.md) |
| 6.3 | [Unit Tests — The Testing Chamber](03_unit_tests.md) |
| 6.4 | [Integration & Doc Tests — Trials from the Outside](04_integration_doc_tests.md) |
| 6.5 | [Documentation — The Archivist's Scrolls](05_documentation.md) |
| 6.6 | [Profiles & Features — The Forge Settings](06_features.md) |
| 🏗️ | [Project: Grimoire Organization](07_project_grimoire_organization.md) |
