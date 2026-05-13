# Rust Vectors Problem Set for Beginner Bioinformaticians

Welcome, young rustacean 🦀

This problem set is designed to teach you how to work with vectors (`Vec<T>`) through biologically inspired exercises. The problems gradually increase in difficulty and mimic real patterns you’ll encounter in bioinformatics and data analysis.

---

# Learning Goals

By the end, you should feel comfortable with:

- Creating vectors
- Accessing elements
- Iterating over vectors
- Mutable vectors
- Pushing/removing elements
- String vectors
- Nested vectors
- Basic algorithms on biological data
- Ownership considerations with vectors

---

# Part 1 — Getting Comfortable with Vectors

---

## Problem 1 — Store DNA Bases

Create a vector containing the DNA bases:

```text
A T G C
```

Print:

1. The whole vector
2. The first base
3. The last base

### Concepts

- `Vec`
- indexing
- `.len()`

### Expected Output (example)

```text
Bases: ['A', 'T', 'G', 'C']
First base: A
Last base: C
```

---

## Problem 2 — RNA Fragment Lengths

You sequenced 5 RNA fragments with lengths:

```text
21, 19, 22, 18, 25
```

Store them in a vector and:

1. Print each fragment length
2. Calculate the total number of nucleotides sequenced
3. Calculate the average fragment length

### Concepts

- iteration
- summation
- integer → float conversion

### Hint

You may need:

```rust
as f64
```

---

## Problem 3 — Count GC Bases

Given this DNA sequence stored as characters:

```text
A T G G C A C C T
```

Count how many bases are:

- G
- C

Then calculate GC content:

```text
(G + C) / total_bases
```

### Concepts

- conditional logic
- counters
- iteration

---

# Part 2 — Mutable Vectors

---

## Problem 4 — Build a Sequence Dynamically

Start with an empty vector.

Add these bases one at a time:

```text
A T G C G G A
```

Print the vector after each insertion.

### Concepts

- `Vec::new()`
- `.push()`
- mutable vectors

---

## Problem 5 — Remove Low Quality Reads

You have read quality scores:

```text
30, 28, 10, 35, 12, 40, 31
```

Remove all scores below 20.

### Challenge

Do this:

1. By creating a new filtered vector
2. Using `.retain()`

### Concepts

- filtering
- mutable operations
- closures

### Hint

Look up:

```rust
retain()
```

---

## Problem 6 — Reverse Complement (Simplified)

Given a DNA sequence:

```text
A T G C C A
```

Create a new vector containing the complementary bases:

| Original | Complement |
|---|---|
| A | T |
| T | A |
| G | C |
| C | G |

Then reverse the complemented sequence.

### Final Result Example

```text
Original:      A T G C C A
Reverse Comp:  T G G C A T
```

### Concepts

- creating new vectors
- matching values
- reversing vectors

### Hint

You may want:

```rust
match
```

and

```rust
.reverse()
```

---

# Part 3 — Strings and Biological Sequences

---

## Problem 7 — Store Multiple Genes

Store these gene names in a vector:

```text
TP53
EGFR
MYC
BRCA1
```

Print each gene on its own line.

### Concepts

- `Vec<String>`
- iteration over strings

### Challenge

Print only genes whose names contain the letter:

```text
R
```

---

## Problem 8 — Find the Longest Sequence

Given several DNA sequences:

```text
"ATGC"
"ATGCGCGT"
"AT"
"ATGCC"
```

Determine:

1. Which sequence is longest
2. Its length

### Concepts

- string length
- tracking maximum values

---

## Problem 9 — Convert DNA to RNA

Convert this DNA sequence:

```text
"ATGCTTACG"
```

into RNA by replacing:

```text
T → U
```

### Expected Output

```text
AUGCUUACG
```

### Concepts

- iterating over characters
- building strings

---

# Part 4 — Nested Vectors and Matrices

---

## Problem 10 — Expression Matrix

Imagine RNA-seq counts from 3 genes across 4 cells:

| Gene | Counts |
|---|---|
| Gene1 | 10 12 8 15 |
| Gene2 | 3 5 1 2 |
| Gene3 | 20 18 25 30 |

Store this as a nested vector.

Print:

1. Each row
2. Total counts per gene
3. Total counts per cell

### Concepts

- nested vectors
- double loops
- indexing

---

## Problem 11 — Mean Expression per Gene

Using the expression matrix from Problem 10:

Calculate the mean expression for each gene.

### Expected Output

```text
Gene1 mean: 11.25
Gene2 mean: 2.75
Gene3 mean: 23.25
```

### Concepts

- nested iteration
- floating point math

---

# Part 5 — Real Bioinformatics Thinking

---

## Problem 12 — k-mer Counting (Beginner Version)

Given the DNA sequence:

```text
ATGATCATG
```

Count all 2-mers.

Example 2-mers:

```text
AT
TG
GA
```

Store counts in vectors.

### Stretch Goal

Use a `HashMap`.

### Concepts

- sliding windows
- substrings
- counting patterns

---

## Problem 13 — Simulate Sequencing Coverage

You have coverage values across a gene:

```text
10, 12, 15, 18, 5, 2, 20, 22, 25
```

Find:

1. Mean coverage
2. Maximum coverage
3. Positions with coverage below 10

### Concepts

- statistics
- iteration with indices

### Hint

Look up:

```rust
enumerate()
```

---

## Problem 14 — Consensus Sequence

You have aligned bases from 4 sequencing reads:

| Position | Bases |
|---|---|
| 1 | A A A G |
| 2 | T T C T |
| 3 | G G G G |
| 4 | C C C T |

Determine the consensus base at each position.

### Expected Output

```text
ATGC
```

### Concepts

- nested vectors
- counting frequencies
- majority voting

---

# Part 6 — Ownership and More Rustic Thinking

---

## Problem 15 — Moving vs Borrowing Vectors

Create a vector of gene names.

Pass it into a function that prints all genes.

Observe what happens when you try to use the vector afterward.

Then modify the function so the vector can still be used after the function call.

### Concepts

- ownership
- borrowing
- references

---

## Problem 16 — Mutating Through References

Create a mutable vector of read counts:

```text
10, 20, 30, 40
```

Write a function that doubles every count in-place.

### Expected Output

```text
20, 40, 60, 80
```

### Concepts

- mutable references
- iterating mutably

### Hint

You’ll probably need:

```rust
iter_mut()
```

---

## Problem 17 — FASTQ-lite Parser

Given a vector of sequencing reads:

```text
"ATGC"
"ATGCGG"
"A"
"AT"
```

Filter out reads shorter than 3 bases.

Then calculate:

1. Number of remaining reads
2. Average read length

### Concepts

- vector filtering
- ownership
- string lengths

---

# Challenge Problems

---

## Challenge 1 — Translate DNA Codons

Given this RNA sequence:

```text
AUGGCUUAA
```

Split it into codons:

```text
AUG
GCU
UAA
```

Translate:

| Codon | Amino Acid |
|---|---|
| AUG | M |
| GCU | A |
| UAA | STOP |

### Expected Output

```text
MA
```

### Concepts

- chunking vectors
- matching strings
- biological translation

---

## Challenge 2 — Mini Differential Expression

You have gene expression counts for two conditions:

| Gene | Control | Treated |
|---|---|---|
| TP53 | 100 | 250 |
| EGFR | 80 | 70 |
| MYC | 40 | 120 |

Calculate fold-change:

```text
treated / control
```

Report genes with fold-change > 2.

### Concepts

- multiple vectors
- indexing
- conditionals

---

## Challenge 3 — Build a Tiny Alignment Matrix

Store pairwise sequence alignments as nested vectors.

Example:

```text
A T G C
A T - C
```

Count:

1. Matches
2. Mismatches
3. Gaps

### Concepts

- nested vectors
- biological comparisons
- algorithmic thinking

---

# Bonus Exploration

Once comfortable, explore:

- `.iter()`
- `.iter_mut()`
- `.into_iter()`
- `.map()`
- `.filter()`
- `.collect()`
- `HashMap`
- slices (`&vec[start..end]`)
- `windows()`

---

# Suggested Progression

| Level | Focus |
|---|---|
| Beginner | Problems 1–6 |
| Comfortable | Problems 7–11 |
| Intermediate | Problems 12–17 |
| Stretch | Challenge Problems |

---

# Rustic Bioinformatics Wisdom 🦀

In biology:

- sequences are vectors
- expression matrices are nested vectors
- coverage tracks are vectors
- alignments are vectors
- k-mers are sliding windows over vectors

Learning vectors well is one of the most important steps toward writing performant bioinformatics software in Rust.

Good luck, rustacean.