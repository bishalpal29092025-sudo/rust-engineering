# Lesson 02 - Variables, Mutability, Shadowing & Constants

**Version:** 1.0

---

# Introduction

Variables are one of the most fundamental concepts in programming. A variable is a named location in memory that stores a value.

Rust takes a unique approach by making variables **immutable by default**. This design helps prevent accidental changes, making programs safer and easier to maintain.

In this lesson, you will learn how to:

* Declare variables
* Use immutable and mutable variables
* Understand shadowing
* Declare constants
* Follow Rust naming conventions

---

# Learning Objectives

After completing this lesson, you should be able to:

* Declare variables using `let`
* Explain why Rust variables are immutable by default
* Modify variables using `mut`
* Explain the difference between mutability and shadowing
* Declare constants using `const`
* Identify when to use `let`, `let mut`, and `const`

---

# Topics Covered

* Variables
* Immutable Variables
* Mutable Variables
* Shadowing
* Constants
* Naming Conventions

---

# 1. Variables

Variables are declared using the `let` keyword.

```rust
let name = "Bishal";
```

Rust automatically infers the variable's type in many cases.

---

# 2. Immutable Variables

By default, variables cannot be changed after they are created.

```rust
let age = 24;

// age = 25; ❌ Compiler Error
```

This helps prevent accidental modifications.

---

# 3. Mutable Variables

If a value needs to change, declare it with the `mut` keyword.

```rust
let mut age = 24;

age = 25;
```

Output:

```text
25
```

Use mutable variables only when modification is required.

---

# 4. Shadowing

Shadowing creates a **new variable** with the same name.

```rust
let score = 50;

let score = score + 25;

let score = score * 2;

println!("{}", score);
```

Output:

```text
150
```

Unlike `mut`, shadowing creates a new binding instead of modifying the existing variable.

---

# 5. Constants

Constants are declared using the `const` keyword.

```rust
const MAX_USERS: u32 = 100;
```

Rules:

* Type annotation is required.
* Value cannot change.
* Name should use `SCREAMING_SNAKE_CASE`.

---

# Comparison

| Feature            | `let`        | `let mut`    | `const`                |
| ------------------ | ------------ | ------------ | ---------------------- |
| Mutable            | ❌            | ✅            | ❌                      |
| Can Reassign       | ❌            | ✅            | ❌                      |
| Type Inference     | ✅            | ✅            | ❌                      |
| Compile-Time Value | ❌            | ❌            | ✅                      |
| Naming Style       | `snake_case` | `snake_case` | `SCREAMING_SNAKE_CASE` |

---

# Best Practices

* Prefer immutable variables.
* Use `mut` only when necessary.
* Use shadowing when transforming values.
* Use constants for values that never change.
* Follow Rust naming conventions.

---

# Common Mistakes

* Forgetting to use `mut` before modifying a variable.
* Confusing shadowing with mutability.
* Forgetting the type when declaring constants.
* Using constants for values that are only known at runtime.

---

# Interview Questions

1. What is a variable?
2. Why are Rust variables immutable by default?
3. What is the difference between `mut` and shadowing?
4. What is the difference between `let` and `const`?
5. Why does Rust encourage immutability?

---

# Summary

In this lesson you learned:

* Variables
* Immutable Variables
* Mutable Variables
* Shadowing
* Constants

These concepts form the foundation for Rust's ownership and borrowing system.

---

# Homework

* Create 10 variables.
* Create 5 constants.
* Practice shadowing.
* Predict program output before running it.
* Explain the difference between `mut` and shadowing in your own words.

---

# Next Lesson

**Lesson 03 – Data Types**

Topics:

* Integers
* Floating Point Numbers
* Booleans
* Characters
* Tuples
* Arrays
