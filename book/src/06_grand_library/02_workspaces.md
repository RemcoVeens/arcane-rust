# ⚔️ 6.2 — Workspaces: The Citadel of Many Towers

> *"A citadel has many towers. The library, the armory, the observatory — each manages itself, yet they share one wall, one well, one gate. Workspaces let you build crates the same way."*

## ⚡ The Concept

### You've been in a workspace all along

Every quest in this course is its own package, yet you run them all from one repository with one `Cargo.lock` and one `target/` folder. That's a **workspace**. Look at the root `Cargo.toml`:

```toml
[workspace]
members = ["exercises/*/*"]     # every quest folder is a member (globs allowed)
exclude = ["legacy"]
resolver = "3"

[workspace.package]             # settings members can inherit
edition = "2024"
version = "0.1.0"
publish = false

[workspace.dependencies]        # dependency versions members can inherit
tokio = { version = "1", features = ["full"] }
```

Note what's missing: a `[package]` section. This root is a **virtual manifest** — it builds nothing itself; it just ties members together.

Each member opts in to the shared settings:

```toml
[package]
name = "ch06_02_workspaces"
version.workspace = true        # ← "whatever the workspace says"
edition.workspace = true
publish.workspace = true

[dependencies]
tokio.workspace = true          # same version for every member that wants it
```

### What a workspace gives you

| Shared | Why it matters |
|---|---|
| One `Cargo.lock` | every member uses the *same* version of each dependency |
| One `target/` | a dependency compiled once is reused by all members |
| `[workspace.package]` / `[workspace.dependencies]` | bump a version in one place |
| `cargo test -p <name>` | build/test one member from anywhere in the repo |
| `cargo test --workspace` | build/test everything |

<div class="compare">

🐍 Python: closest is a monorepo with several packages and a shared lock file (uv/Poetry workspaces). · 🐹 Go: `go.work` with `use ./a ./b` — very similar idea. · 🦀 Rust: `[workspace]` in the root `Cargo.toml`, built into Cargo since forever.

</div>

### Path dependencies inside the workspace

A crate can depend on a sibling member with a path dependency:

```toml
[dependencies]
arcana_core = { path = "arcana_core" }
```

A path dependency that lives inside the workspace folder automatically **becomes a member**, even if `members` doesn't list it. That's how `arcana_core/` (nested inside this quest) joins the workspace — and why it must follow the workspace's rules too.

### The API boundary between crates

Inside one crate, privacy is mostly a matter of tidiness. **Between crates it's a hard wall**: a sibling crate sees only what's `pub` *and* reachable through `pub` modules or `pub use`. Splitting a project into crates forces you to design that public surface deliberately:

```rust,ignore
// arcana_core/src/lib.rs
mod elements;               // private module: an implementation detail…
pub use elements::Element;  // …but its Element enum is part of the public API
```

Now users write `arcana_core::Element`, and you're free to reorganise the files later without breaking anyone.

### Compile-time metadata

`env!("CARGO_PKG_VERSION")` reads the package's version from `Cargo.toml` **at compile time** — handy for `--version` output, and a nice way to check what a crate actually inherited.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Don't put a `[package]` in a virtual workspace root.** Members each have their own `[package]`.
- **Forgetting `.workspace = true`** means a member silently keeps its own value. Nothing fails — until two crates disagree on a version or edition.
- **Nested packages outside the workspace folder** (or not reachable via `members` or a path dependency) make Cargo complain: "current package believes it's in a workspace when it's not".
- **`pub mod` is not the only fix.** Making a whole module public exposes everything in it; `pub use` exposes exactly the items you choose.
- **`-p` takes the package name** (`ch06_02_workspaces`), not the folder path.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/02_workspaces`

The crate depends on its neighbour `arcana_core/`, whose API isn't ready. Fix **arcana_core**:

1. Re-export `Element` so that `use arcana_core::Element;` works (keep `elements` private).
2. Make `ManaCrystal::drain` callable from other crates.
3. Make `arcana_core` inherit its version from the workspace, so `arcana_core::VERSION == "0.1.0"`.

Then implement `counter_element` and `core_banner` in `src/lib.rs`.

</div>

Bonus: from the repo root, run `cargo test -p arcana_core` (it has no tests, but it builds) and `cargo tree -p ch06_02_workspaces`.

## 📜 The Grimoire

<div class="grimoire">

- Root: `[workspace] members = [...]` + `[workspace.package]` + `[workspace.dependencies]`
- Member: `version.workspace = true`, `dep.workspace = true`
- One `Cargo.lock`, one `target/` for all members
- In-workspace path deps become members automatically
- `pub use module::Item;` = choose your public API · `env!("CARGO_PKG_VERSION")`
- `cargo test -p <pkg>` · `cargo test --workspace`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the errors</summary>

`unresolved import arcana_core::Element` → the enum exists, but only at `arcana_core::elements::Element`, which is private. `method drain is private` → look at its signature. Both fixes go in `arcana_core/src/lib.rs`.

</details>
<details class="hint"><summary>Hint 2 — the version</summary>

Open `arcana_core/Cargo.toml` and compare it with this quest's `Cargo.toml`. One of these lines is not like the others.

</details>
<details class="hint"><summary>Hint 3 — counter_element</summary>

You need the element `x` with `x.strong_against() == enemy`. A `match enemy { Element::Frost => Element::Fire, … }` is perfectly fine; or notice that the wheel has three steps, so walking it twice from `enemy` lands on the answer.

</details>
