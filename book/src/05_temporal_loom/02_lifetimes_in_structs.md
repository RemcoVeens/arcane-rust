# ⚔️ 5.2 — Lifetimes in Structs: The Crystal Ball

> *"A crystal ball shows a vision of something that exists elsewhere. It holds no castle, no army — only a window onto them. Shatter the castle, and the vision must go dark first."*

## ⚡ The Concept

So far every struct you wrote **owned** its data (`String`, `Vec<T>`). A struct can also **borrow** — and then it must say how long the borrowed data lives.

```rust,compile_fail
struct ScrollReader {
    text: &str,   // ❌ missing lifetime specifier
}
```

```rust
struct ScrollReader<'a> {
    text: &'a str,   // "I borrow a str that lives at least 'a"
    pos: usize,
}
```

`ScrollReader<'a>` reads: *a reader that cannot outlive the text it reads*. The compiler now enforces that every `ScrollReader` is dropped before its scroll.

### `impl` blocks carry the lifetime too

```rust
# struct ScrollReader<'a> { text: &'a str, pos: usize }
impl<'a> ScrollReader<'a> {
    fn new(text: &'a str) -> Self {
        ScrollReader { text, pos: 0 }
    }

    fn remaining(&self) -> &'a str {
        &self.text[self.pos..]
    }
}
```

`impl<'a>` declares the parameter; `ScrollReader<'a>` uses it — exactly like `impl<T> Wrapper<T>`.

### `&self` vs `'a` — the crucial choice

Look closely at `remaining`. Its result is `&'a str`, **not** `&str`. What's the difference?

```rust,ignore
fn remaining(&self) -> &str      // elided: borrows from the *reader* (rule 3)
fn remaining(&self) -> &'a str   // explicit: borrows from the *scroll*
```

With the elided version, the returned line keeps the *reader* borrowed. That's too strict:

```rust,ignore
let first = reader.next_line();   // borrows `reader` (if elided)…
let second = reader.next_line();  // ❌ …so it can't be borrowed mutably again
println!("{first:?}");
```

Returning `&'a str` says the truth: the line lives inside the scroll, and the reader is just a cursor. You can hold many lines, call `next_line` again, even drop the reader. This is the same trick `str::split` and friends use.

### Structs in results

Lifetimes nest naturally:

```rust
#[derive(Debug)]
struct Highlight<'a> {
    line: &'a str,
    line_number: usize,
}

fn find<'a>(scroll: &'a str, word: &str) -> Vec<Highlight<'a>> {
    let mut found = Vec::new();
    for (i, line) in scroll.lines().enumerate() {
        if line.contains(word) {
            found.push(Highlight { line, line_number: i + 1 });
        }
    }
    found
}
```

The highlights borrow from `scroll` — never from `word` — so the search word can be dropped right after the call.

<div class="compare">

🐍 Python: a "view" object (like `memoryview` or a dict view) keeps the original alive via reference counting. · 🐹 Go: a struct holding a slice keeps the backing array alive via the GC. · 🦀 Rust: a struct holding a reference keeps *nothing* alive — instead the compiler forbids the struct from outliving what it borrows. Zero cost, checked at compile time.

</div>

### Owned or borrowed?

| Use owned (`String`) when… | Use borrowed (`&'a str`) when… |
|---|---|
| the struct lives a long time / is stored | it's a short-lived view or cursor |
| you build or modify the data | you only read existing data |
| you want simple code | you want zero-copy parsing |

When in doubt, own. Borrowing structs shine for parsers, readers and iterators.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Returning `&str` from a method borrows `self`.** If the data really lives in the borrowed source, return `&'a str` explicitly.
- **Slicing by byte offsets:** `&text[a..b]` panics if `a`/`b` isn't on a UTF-8 character boundary. Offsets from `find('\n')` are always safe.
- **Don't make long-lived structs borrow.** A `Game` that holds `&'a Mage` infects everything that holds a `Game` with `'a`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/02_lifetimes_in_structs`

1. Add lifetime parameters to `ScrollReader` and `Highlight` (and to the `impl` and `find_highlights`).
2. Implement `new`, `next_line`, `remaining` and `find_highlights`.

The test `lines_outlive_the_next_read` only compiles if `next_line` returns a line tied to the **scroll**, not to the reader.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `struct S<'a> { field: &'a T }` · `impl<'a> S<'a> { ... }`
- Method returning `&'a T` → borrows the source; `&T` → borrows `self`
- `s.find('\n')` → `Option<usize>` byte offset · `&s[start..]` · `&s[..end]`
- `for (i, line) in text.lines().enumerate()`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

`pub struct ScrollReader<'a> { text: &'a str, pos: usize }` and `impl<'a> ScrollReader<'a>`. `new` takes `text: &'a str`.

</details>
<details class="hint"><summary>Hint 2</summary>

Implement `remaining` first: `&self.text[self.pos..]` with return type `&'a str`. Then `next_line` can start with `let rest = self.remaining();` — `rest` is a `&'a str`, so slices of it are too.

</details>
<details class="hint"><summary>Hint 3</summary>

In `next_line`: if `rest` is empty → `None`. If `rest.find('\n')` is `Some(end)`, advance `pos` by `end + 1` and return `&rest[..end]`. Otherwise advance `pos` to `text.len()` and return all of `rest`.

</details>
