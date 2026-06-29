# Lesson 02 – Interview Questions & Answers

## Version

1.0

---

# Question 1

## What is a variable in Rust?

### Answer

A variable is a named location in memory used to store a value.

Variables in Rust are declared using the `let` keyword.

Example:

```rust
let name = "Bishal";
```

By default, variables are immutable.

---

# Question 2

## Why are variables immutable by default?

### Answer

Rust makes variables immutable by default to:

* Prevent accidental modifications
* Improve code safety
* Make programs easier to understand
* Reduce bugs caused by unexpected state changes

If a variable needs to change, the programmer must explicitly use the `mut` keyword.

---

# Question 3

## What is the difference between `let` and `let mut`?

### Answer

`let`

* Immutable
* Cannot change the value

Example

```rust
let age = 24;
```

`let mut`

* Mutable
* Value can change

Example

```rust
let mut age = 24;

age = 25;
```

---

# Question 4

## What is shadowing?

### Answer

Shadowing is the process of creating a new variable with the same name as an existing variable.

Example

```rust
let number = 10;

let number = number + 10;
```

The second `number` is a completely new variable that hides the previous one.

---

# Question 5

## How is shadowing different from mutability?

### Answer

Mutability changes the value of the same variable.

Example

```rust
let mut score = 10;

score = 20;
```

Shadowing creates a new variable.

Example

```rust
let score = 10;

let score = score + 10;
```

---

# Question 6

## Why would you use shadowing instead of `mut`?

### Answer

Shadowing is useful when:

* Transforming values
* Changing variable types
* Keeping variables immutable
* Writing cleaner and more readable code

Example

```rust
let input = "42";

let input = input.parse::<i32>().unwrap();
```

The variable changes from a string to an integer.

---

# Question 7

## What is a constant?

### Answer

A constant is a value that:

* Never changes
* Must have a type annotation
* Is evaluated at compile time

Example

```rust
const MAX_USERS: u32 = 100;
```

---

# Question 8

## Difference between `let` and `const`

| let              | const                 |
| ---------------- | --------------------- |
| Runtime variable | Compile-time constant |
| Type inference   | Type required         |
| Can be mutable   | Always immutable      |

---

# Question 9

## Why does `const` require a type?

### Answer

The Rust compiler needs to know the exact type of a constant during compilation.

Unlike `let`, Rust does not infer the type of constants.

---

# Question 10

## Explain this code.

```rust
let number = 5;

let number = number + 5;

let number = number * 2;
```

### Answer

1. First variable = 5

2. Shadowing creates a new variable = 10

3. Shadowing creates another new variable = 20

Final output:

```text
20
```

---

# Question 11

## Explain the output.

```rust
let mut age = 24;

println!("{}", age);

age = 25;

println!("{}", age);
```

### Answer

Output

```text
24
25
```

The first print occurs before the value changes.

The second print occurs after reassignment.

---

# Question 12

## What naming conventions are used in Rust?

Variables

```rust
let student_name = "Bishal";
```

Uses **snake_case**

Constants

```rust
const MAX_USERS: u32 = 100;
```

Uses **SCREAMING_SNAKE_CASE**

---

# Common Follow-Up Questions

* Why is Rust considered memory safe?
* Why does Rust encourage immutability?
* Can shadowing change the variable type?
* Can constants be mutable?
* Why does Rust have both `mut` and shadowing?

---

# Interview Tips

When answering Rust interview questions:

* Explain **what** the feature is.
* Explain **why** Rust designed it that way.
* Give a short code example.
* Mention a real-world use case if possible.

A complete answer is much stronger than only defining the keyword.
