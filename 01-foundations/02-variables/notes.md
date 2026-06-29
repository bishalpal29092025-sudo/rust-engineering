# Lesson 02 – Quick Revision Notes

## Variables

```rust
let name = "Bishal";
```

* Declared using `let`
* Immutable by default
* Type is usually inferred automatically

---

## Mutable Variables

```rust
let mut age = 24;

age = 25;
```

* `mut` allows the value to change.
* Use only when necessary.

---

## Shadowing

```rust
let number = 10;

let number = number + 10;
```

* Creates a **new variable** with the same name.
* Does **not** modify the original variable.
* Useful for transforming values or changing types.

---

## Constants

```rust
const MAX_USERS: u32 = 100;
```

Rules:

* Must include a type.
* Cannot be modified.
* Uses `SCREAMING_SNAKE_CASE`.

---

## Naming Convention

Variables:

```rust
let student_name = "Bishal";
```

Constants:

```rust
const MAX_USERS: u32 = 100;
```

---

## Quick Comparison

| Keyword   | Purpose               |
| --------- | --------------------- |
| `let`     | Immutable variable    |
| `let mut` | Mutable variable      |
| `const`   | Compile-time constant |

---

## Keywords Learned

* `let`
* `mut`
* `const`
* `println!`

---

## Common Errors

* Trying to modify an immutable variable.
* Forgetting `mut`.
* Forgetting the type when declaring a constant.
* Confusing shadowing with mutability.

---

## Interview One-Liners

* Variables in Rust are **immutable by default**.
* `mut` changes the same variable.
* Shadowing creates a **new binding**.
* Constants require a type annotation and are evaluated at compile time.

---

## Remember

> Prefer immutable variables. Use `mut` only when necessary. Use shadowing when creating a transformed value. Use constants for values that never change.
