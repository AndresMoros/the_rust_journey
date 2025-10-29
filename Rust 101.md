# Variables
The variables on Rust can be declared with the keyword `let`. The name's convention is the same that in other languages. No numbers at the variable's name beginning, and it doesn't allows the use of special characters on it. Example:

```rust
let name = "Andrés";
```

## Mutability
By default, all variables on Rust are inmutables, that mean we can't change their value during the program. In order to change this, we have to use the `mut` keyword at the declaration moment. For example:

 ```rust
 let mut age: u32 = 22;
 ```

Now we can change the value of `age` later.

```rust
let mut age: u32 = 22;
age = 27;
```

# Constants
The constants in Rust are inmutables, and their values can't change during the program. Other difference between constants and variables are how you can declare them: using the keyword `const` and you should specify the type. 

## Declaring constants
The actual name convention for variables are all the letters in `UPPERCASE` and using underscores (`_`) as separator. For example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The constants can be declare in any scope (global or within a function), and their values HAS to be know, that means the value of a constant cannot be the result of a calculation in run time. How to **NOT** declare a constant:

```rust
const USER_COUNT: u32 = get_user_count(); // NOT ALLOWED
```

At the practice, you'll be able to access the constants at any point off your program, that makes them useful to values that never changes like `MAX_POINTS`, `SPEED_OF_LIGHT` or limits/configurations. At the same time, if you need to change their value you only have to do it in one place. 

#rust #software_development