# Rust Practice Problems: Chapters 1–4

A biology-themed problem set for beginner Rustaceans who have finished Chapter 4 (Understanding Ownership) of *The Rust Programming Language*. Problems progress from easy to challenging and reinforce:

- **Ch 1–2**: `cargo`, `println!`, basic I/O, variables
- **Ch 3**: Data types, functions, control flow, loops
- **Ch 4**: Ownership, references & borrowing, slices

Try to solve each problem without peeking at the hints. The goal isn't just to get a working answer — it's to *feel* why the borrow checker is happy (or angry).

---

## Level 1: Warm-ups (Ch 1–3)

### Problem 1.1 — GC Content, Hardcoded
Write a program with a hardcoded DNA sequence (e.g., `"ATCGGCTA"`) stored in a variable. Print the sequence, then print its length.

> **Concepts**: `fn main`, `let`, `println!` with `{}`, `.len()` on a string.

### Problem 1.2 — Complement a Base
Write a function `complement(base: char) -> char` that returns the Watson-Crick complement of a single DNA base (`A↔T`, `C↔G`). Use a `match` expression. Call it from `main` on a few bases and print the results.

> **Concepts**: functions, `char` type, `match`, return values without `;`.

### Problem 1.3 — Codon to Amino Acid (Start + Stop Only)
Write a function that takes a 3-character codon as a `&str` and returns a `&'static str`:
- `"ATG"` → `"Met (Start)"`
- `"TAA"`, `"TAG"`, `"TGA"` → `"Stop"`
- anything else → `"Other"`

Use `match` with multiple patterns (`"TAA" | "TAG" | "TGA" => ...`).

> **Concepts**: string literals, `match` with `|`, `&str` parameters.

### Problem 1.4 — Count the Gs
Given a hardcoded DNA string, use a `for` loop over `.chars()` to count how many `G` bases it contains. Print the count.

> **Concepts**: `for` loops, `.chars()` iterator, mutable variables (`let mut`).

---

## Level 2: Control Flow & Functions (Ch 3)

### Problem 2.1 — GC Content as a Percentage
Write `fn gc_content(seq: &str) -> f64` that returns the fraction of `G` and `C` bases (0.0 to 1.0). Handle the empty-string case by returning `0.0`. Print the result for `"ATCGGGCTA"` formatted to 2 decimal places (`{:.2}`).

> **Concepts**: casting with `as f64`, early return with `if`, float formatting.

### Problem 2.2 — Transcribe DNA to RNA
Write `fn transcribe(dna: &str) -> String` that returns a new `String` where every `T` is replaced by `U`. Do this by pushing chars into a new `String` inside a loop (don't use `.replace()` — the point is to practice building a `String`).

> **Concepts**: `String` vs `&str`, `String::new()`, `.push(char)`, returning owned data.

### Problem 2.3 — Find the First Stop Codon
Given a DNA sequence, scan it **in frame** (codons at positions 0, 3, 6, ...) and return the index of the first stop codon (`TAA`, `TAG`, `TGA`), or `-1` if none is found. Use a `while` loop and slice indexing like `&seq[i..i+3]`.

Signature: `fn first_stop(seq: &str) -> i32`

> **Concepts**: `while` loops, string slices `&str[..]`, returning from loops. (Note: we're using `i32` with `-1` as a sentinel here because `Option` is Ch 6 — when you get there, you'll see why `Option<usize>` is better.)

---

## Level 3: Ownership & Borrowing (Ch 4)

### Problem 3.1 — Who Owns the Sequence?
Write a function `fn print_length(seq: String)` that prints `seq.len()`. In `main`, create a `String` with `String::from("ACGT")`, call `print_length(seq)`, and then try to print `seq` again after the call.

**It won't compile.** Read the error carefully. Then fix it in **two different ways**:
1. By making the function take `&String` (or better, `&str`) instead.
2. By `.clone()`-ing the string at the call site.

Write a short comment in your code explaining which fix is better and why.

> **Concepts**: move semantics, the difference between borrowing and cloning, why borrowing is usually preferred.

### Problem 3.2 — Reverse Complement (Borrow In, Own Out)
Write `fn reverse_complement(dna: &str) -> String`. It should take a borrowed string slice and return a newly-owned `String`.

For example, `reverse_complement("ATCG")` returns `"CGAT"`.

Hint: iterate with `.chars().rev()`, push complemented bases into a new `String`.

> **Concepts**: the "borrow input, own output" pattern you'll use constantly in Rust.

### Problem 3.3 — Longest ORF-ish Slice
Write `fn longest_run<'a>(seq: &'a str, base: char) -> &'a str` that returns a **slice** of the input referring to the longest consecutive run of `base` in `seq`. For example, `longest_run("ATTTGCAAAAT", 'A')` returns `"AAAA"`.

You do **not** allocate a new `String` — you return a slice into the original. This is the whole point of the exercise.

> **Concepts**: slices as borrows, lifetime elision vs. explicit lifetimes, why returning a slice is zero-copy. The `'a` annotation may or may not be required depending on how you write it — try both.

### Problem 3.4 — Mutable Borrow: Mask Low-Quality Bases
Write `fn mask_base(seq: &mut String, index: usize)` that replaces the character at `index` with `'N'`. Since `String` indexing by byte is tricky, assume ASCII-only DNA and use:

```rust
unsafe { seq.as_bytes_mut()[index] = b'N'; }
```

…OR, more safely, rebuild the `String` with the replacement. Try both approaches and think about why the safe version is preferred.

In `main`, create a `let mut seq = String::from("ACGTACGT");`, call `mask_base(&mut seq, 3)`, and print the result. Then try to create a second mutable borrow at the same time and watch the borrow checker complain.

> **Concepts**: `&mut T`, the "one mutable borrow XOR many immutable borrows" rule.

### Problem 3.5 — The Dangling Reference Trap
Try to write this function:

```rust
fn make_primer() -> &str {
    let s = String::from("ATCGATCG");
    &s[0..6]
}
```

It won't compile. Read the error, understand *why* (the `String` is dropped at the end of the function, so the slice would dangle), and then write **two fixes**:
1. Return `String` instead of `&str`.
2. Take the source string as a parameter and return a slice into it.

> **Concepts**: dangling references, why Rust forbids them at compile time, the relationship between ownership and lifetimes.

---

## Level 4: Putting It Together

### Problem 4.1 — Mini Sequence Stats
Write a program that defines a hardcoded DNA sequence and prints:
- Length
- Count of each base (A, C, G, T, N/other)
- GC content as a percentage
- The reverse complement
- Whether the sequence is a palindrome (equal to its reverse complement — real restriction sites like `GAATTC` are!)

Organize your code into small functions, each taking `&str` where possible. `main` should read like a summary of what the program does.

> **Concepts**: composing borrowed functions, returning owned vs. borrowed data appropriately, keeping `main` clean.

### Problem 4.2 — In-Frame Translation (Start to Stop)
Write `fn translate_orf(dna: &str) -> String` that:
1. Finds the first `ATG` in the sequence (use `.find("ATG")` — it returns `Option<usize>`, which is Ch 6, so just `.unwrap()` it for now or return `String::new()` if absent).
2. From that position, reads codons one at a time.
3. Translates each codon to a single-letter amino acid using a helper `fn codon_to_aa(codon: &str) -> char`. You only need to handle a handful of codons plus `*` for stop — don't write all 64.
4. Stops at the first stop codon and returns the protein string.

> **Concepts**: all of Ch 1–4 together. This is the kind of function you'd actually write in bioinformatics Rust code.

---

## How to Check Yourself

For each problem, ask:
1. **Could I have used `&str` instead of `String`?** (Usually yes, for function parameters.)
2. **Am I cloning unnecessarily?** (Clones are fine while learning, but notice them.)
3. **Does the borrow checker's error message make sense to me?** If not, re-read the relevant section of Ch 4 — the errors are trying to teach you something specific.

When Ch 5 (structs) and Ch 6 (enums & `Option`) click, come back to these problems and rewrite them using `Option<usize>` instead of `-1`, and a `struct Sequence` instead of loose `&str` parameters. You'll feel the difference.

Good luck, and happy borrowing!
