# Exercise 01 – Variables

## Objective

Practice declaring variables using the `let` keyword and printing their values using `println!`.

---

## Instructions

Create a Rust program that stores the following information in separate variables:

* Name
* Age
* Country
* State
* City
* University
* Course
* Semester
* Favorite Programming Language
* Career Goal

Print all the information in a neat format.

---

## Expected Output

```text
=================================
      Student Information
=================================
Name      : Bishal Pal
Age       : 24
Country   : India
State     : West Bengal
City      : Bankura
University: Adamas University
Course    : MCA
Semester  : 3rd
Language  : Rust
Goal      : Full Stack Solana Developer
=================================
```

---

## Challenge

1. Add two more variables of your own choice.
2. Change the values and run the program again.
3. Observe how only the output changes while the program structure remains the same.

---

## Learning Outcomes

After completing this exercise, you should be able to:

* Declare variables using `let`
* Store different kinds of information
* Print variables using `println!`
* Understand that variables hold values that can be displayed in the program

---

## Bonus Challenge ⭐

Create another section called **Dream Company** and print:

* Dream Company
* Dream Role
* Years of Experience Goal
* Favorite Technology

Format it neatly using `println!`.


```rust

const OWNER: &str = "Bishal Pal";


fn main() {

    let name = "Bishal Pal";
    let age = 24;
    let country = "India";
    let state = "West Bengal";
    let city = "Bankura";
    let university = "Adamas University";
    let course = "MCA";
    let semester = "4th";
    let language = "Rust";
    let goal = "Full Stack Solana Developer";


    println!("=================================");
    println!("      Student Information        ");
    println!("=================================");
    println!("Name      : {}", name);
    println!("Age       : {}", age);
    println!("Country   : {}", country);
    println!("State     : {}", state);
    println!("City      : {}", city);
    println!("University: {}", university);
    println!("Course    : {}", course);
    println!("Semester  : {}", semester);
    println!("Language  : {}", language);
    println!("Goal      : {}", goal);

    let mut age = 24;
    println!("Age: {}", age);

    age = 25;
    println!("Age: {}", age);



    // Shadowing
    let age = 24;
    println!("Age: {}", age);
    let age = 25;
    println!("Age: {}", age);



    let input = "42";
    let input = input.parse::<i32>().unwrap();
    println!("{}", input);


    let number = 5;
    let number = number + 5;
    let number = number * 2;
    println!("{}", number);


    let input = "41";
    let parse_input = input.parse::<i32>().unwrap();
    let double_input = parse_input * 2;

    println!("{}", input);
    println!("{}", parse_input);
    println!("{}", double_input);

    let input = "41";
    let input = input.parse::<i32>().unwrap();
    let input = input * 2;

    println!("Input is {}", input);


    let mut count = 10;
    println!("Count is: {}", count);
    count = 20;
    println!("Count is after change the mutable value: {}", count);
    let count = count + 5;
    println!("Count is now {}", count);


    const MAX_USERS: u32 = 100;
    println!("Maximum users at a time: {}", MAX_USERS);

    const APP_NAME: &str = "Rust Engineering";
    println!("Application Name is {}", APP_NAME);

    println!("Owner name {}", OWNER);

    const PI_VALUE: f64 = 3.147;
    println!("Value of PI is {}", PI_VALUE);
}
```

