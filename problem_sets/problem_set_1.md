# Rust Beginner Practice Problems (Variables → Functions)

These exercises are designed to take ~10–20 minutes total and reinforce:

* Variables and mutability
* Data types
* Functions
* Statements vs expressions
* Return values

---

# 🧪 Problem 1 — Temperature Converter (Warm-up)

## Goal

Write a function that converts Fahrenheit to Celsius.

## Requirements

* Function takes a `f64`
* Returns a `f64`
* Use an **expression-based return** (no `return` keyword)

## Formula

[
C = (F - 32.0) * 5.0 / 9.0
]

## Example usage

```rust
let temp_f = 77.0;
let temp_c = fahrenheit_to_celsius(temp_f);
println!("{}F = {}C", temp_f, temp_c);
```

---

# 🧪 Problem 2 — Even or Odd

## Goal

Write a function that determines whether a number is even.

## Requirements

* Input: `i32`
* Output: `bool`
* Use an **expression**, not a `return` statement

## Example usage

```rust
let num = 42;
println!("Is even? {}", is_even(num));
```

---

# 🧪 Problem 3 — Simple Calculator Function

## Goal

Create a function that takes two numbers and returns both their sum and product.

## Requirements

* Return multiple values using a tuple
* Use expression-based returns

## Example usage

```rust
let (sum, product) = calculate(3, 4);
println!("Sum: {}, Product: {}", sum, product);
```

---

# 🧪 Problem 4 — Mutability & Scope

## Goal

Demonstrate mutability and scope behavior in Rust.

## Task

* Create a mutable variable `x = 5`
* Inside a block `{}`, modify it (e.g., multiply by 2)
* Print `x` inside and outside the block

## Question

Does the value change outside the block?

## Example structure

```rust
let mut x = 5;

{
    // modify x here
    println!("Inside block: {}", x);
}

println!("Outside block: {}", x);
```

---

# 🧪 Problem 5 — Mini Score Classifier (Slightly Harder)

## Goal

Write a function that classifies a score into categories.

## Requirements

* Input: `i32`
* Output: `&'static str`
* Use `if` as an expression

## Rules

* 90+ → "Excellent"
* 75–89 → "Good"
* 50–74 → "Pass"
* <50 → "Fail"

## Example usage

```rust
let score = 82;
let result = classify_score(score);
println!("Result: {}", result);
```

---

# 🔥 Bonus Challenge (Optional)

Modify Problem 5 so that it returns a tuple:

```rust
("Good", true)
```

Where the boolean indicates whether the student passed.

---

# 🧠 What You’re Practicing

* Problem 1 → numeric types + expressions
* Problem 2 → boolean logic
* Problem 3 → tuples + multiple return values
* Problem 4 → mutability + scope behavior
* Problem 5 → `if` as an expression

---

Happy coding 🚀

