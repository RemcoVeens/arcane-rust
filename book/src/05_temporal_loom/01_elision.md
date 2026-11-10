# ⚔️ 5.1 — Lifetime Elision: The Unspoken Components

> *"A master mage does not announce every spell component — some are obvious. 'Eye of newt' goes without saying. The compiler is the same: most lifetimes go without saying."*

## ⚡ The Concept

You have written dozens of functions that take and return references without a single `'a`:

```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

That works because of **lifetime elision**: three rules the compiler applies to fill in the obvious lifetimes for you. If after the rules every output lifetime is known, you write nothing. If not, you get `missing lifetime specifier`.

### The three rules

**Rule 1 — every input reference gets its own lifetime.**

```rust,ignore
fn f(x: &str, y: &str)          // becomes
fn f<'a, 'b>(x: &'a str, y: &'b str)
```

**Rule 2 — exactly one input lifetime? The outputs get it.**

```rust,ignore
fn first_word(s: &str) -> &str  // becomes
fn first_word<'a>(s: &'a str) -> &'a str
```

**Rule 3 — a method with `&self` or `&mut self`? The outputs get `self`'s lifetime.**

```rust,ignore
impl Grimoire {
    fn find(&self, prefix: &str) -> Option<&str>   // becomes
    fn find<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a str>
}
```

Rule 3 encodes the overwhelmingly common case: a method returns something *inside* the struct, not inside some other argument.

### When elision gives up

```rust,compile_fail
fn choose(a: &str, b: &str, first: bool) -> &str {   // ❌
    if first { a } else { b }
}
```

Two input lifetimes (rule 1), no `self` (rule 3 doesn't apply), so rule 2 can't pick one. You must write it out:

```rust
fn choose<'a>(a: &'a str, b: &'a str, first: bool) -> &'a str {
    if first { a } else { b }
}
```

### A quick decision table

| Signature | Elided? |
|---|---|
| `fn f(x: &T) -> &U` | ✅ rule 2 |
| `fn f(x: &T, n: u32) -> &U` | ✅ only one *reference* input |
| `fn f(&self, other: &T) -> &U` | ✅ rule 3 (borrows from `self`) |
| `fn f(x: &T, y: &T) -> &U` | ❌ annotate |
| `fn f() -> &U` | ❌ nothing to borrow from — `'static` or return owned |

<div class="compare">

🐹 Go: `func first(s string) string` returns a new string header pointing into the same bytes — the GC keeps them alive. 🦀 Rust does the same zero-copy slicing, but the borrow checker tracks that the result points into `s`. Elision is why that tracking costs you zero keystrokes most of the time.

</div>

### Rule 3 can surprise you

If a method's result really borrows from an *argument* rather than `self`, rule 3 picks the wrong one and you'll get an error at the call site or in the body. Then annotate explicitly:

```rust
struct Judge;
impl Judge {
    fn pick<'a>(&self, a: &'a str, _b: &str) -> &'a str { a }
}
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Elision is not inference.** The compiler never looks at the function body to decide lifetimes — only at the signature and the three rules.
- **`n: u32` doesn't count.** Only reference parameters have lifetimes.
- **Don't sprinkle `'a` everywhere.** `clippy` will even warn (`needless_lifetimes`) when you write lifetimes elision would have produced.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/01_elision`

The file doesn't compile: exactly **one** signature needs explicit lifetimes. Find it and fix it. Then implement the `todo!()`s — none of them needs a single `'a`.

</div>

Look at the test `finding_outlives_the_prefix`: the `prefix` string is dropped before the result is used. That compiles only because rule 3 ties the result to `self`, not to `prefix`.

## 📜 The Grimoire

<div class="grimoire">

- Rule 1: each input reference gets its own lifetime
- Rule 2: one input lifetime → all outputs get it
- Rule 3: `&self` / `&mut self` → outputs get `self`'s lifetime
- Still ambiguous → `missing lifetime specifier` → annotate
- `s.split_whitespace().next()` · `s.trim_start_matches('✦')` · `s.starts_with(p)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

Count the reference parameters of each free function. The one with two `&str` inputs and no `self` is the one that needs help.

</details>
<details class="hint"><summary>Hint 2</summary>

`first_word`: `split_whitespace()` returns an iterator; `.next()` gives an `Option<&str>`; `.unwrap_or("")` handles the empty case. `title`: `&self.title` (a `&String` auto-derefs to `&str`).

</details>
<details class="hint"><summary>Hint 3</summary>

`longest_spell`: loop over `&self.spells`, keep an `Option<&str>` with the best so far and replace it only when a spell is *strictly* longer. `find`: `self.spells.iter().find(|s| s.starts_with(prefix)).map(|s| s.as_str())`.

</details>
