The ownership system in Rust is that makes it mostly unique comparing it with other programming languages. The ownership are a set of rules that lead how a Rust program manages memory and the data that it works with.

## Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Wa da hell is an _owner_
In Rust, an just how I understand it, an owner is a variable, or the variable's name. So, in order to get access to a value in anytime we needs it in our program we have to holds it on a variable, and we declare and initialize that variable making it the owner of the value we want to use.

## Variable scope
The _scope_ of a variable defines when a variable is avaliable for use it during the execution of our program. On the ownership rules, variables are the owners, and when it goes out oof the scopee, the value is dropped (in other words, it's not longer avaliable), and we can't make a reference to it.

```rust
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```
If after the curly brackets we try to usee `s` again, we'll get a reference error. Pretty similar as other languages.

### Copy trait
