> ```rust
> fn main() {
>     let api_token = std::env::var("API_TOKEN");
>     dbg!(api_token);
> }
> ```
> Semicolons are important in other languages for seemingly arbitrary reasons.
>
> What happens if you remove the semicolon from the `dbg!` call in our program? What do you think the error message is telling you?

# Answers
When we remove the semicolon from the `dbg!` macro we get this error message.

```
❯ cargo build
   Compiling weather-cli v0.1.0 (/Users/chris/weatcher-cli)
error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
1 | fn main() {
  |           - expected `()` because of default return type
...
4 |     dbg!(api_token)
  |     ^^^^^^^^^^^^^^^ expected `()`, found struct `String`
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `thingd`

To learn more, run the command again with --verbose.
```

There are a couple of key insights here.

1. The default return value of a function is `()`
2. Everything in Rust is either a Statement or an Expression
3. `dbg!` returns the value of the expression we pass it
4. Semicolons turn Expressions into Statements

## 1. The default return value of a function is `()`

`()` is pronounced "unit" and is a 0 element tuple. If we don't specify a return type for our functions, they return `()` by default. The type unit `()` and the only value that can satisfy that type `()` are spelled the same way. This means if a function's return type is `()`, then the only value we can return at the end of our function is `()`.

By comparison in JavaScript if a function returns the type "Number", then it would return values like: `1`, `2`, `3`, etc.

## 2. Everything in Rust is either a Statement or an Expression

Rust is primarily an Expression based language. This means that inside of a block there is always a return value from that block. This even applies to `if`.

In JavaScript `if` is a statement, and thus doesn't return a value. This leads us into using hacks like ternary operators, or `mything && MyComponent`.

In Rust, we can return values from an if expression and put them into a variable as such.

```rust
let y = if 12 * 15 > 150 {
    "Bigger"
} else {
    "Smaller"
};
```

We could put this into a function that returns a `String`. Notice how we removed the variable assignment as well as the semi-colon. The last value in an expression will be returned.

```rust
fn test_a_number() -> String {
    if 12 * 15 > 150 {
        "Bigger".to_string()
    } else {
        "Smaller".to_string()
    }
}
```

In this case, that means `"Bigger".to_string()` is returned from one branch of the if expression, and `"Smaller".to_string()` is returned from the other branch of the if expression.

This means that overall the if expression returns a `String`, and since the if expression is the last expression in the function body, the function returns a `String` as well.

## 3. `dbg!` returns the value of the expression we pass it

One unique feature of the `dbg!` macro is that since it is use for debugging purposes, the value returned from it is the value of the expression we pass it. It's as if the `dbg!` macro wasn't even there.

In the following example, we print out the result of `dbg!(2) == 2`, which is the same as `2 == 2`, to prove that dbg passes the value through.

```rust
fn main() {
    println!("{}", dbg!(2) == 2);
}
```

## 4. Semicolons turn Expressions into Statements

So to recap:

- Functions return the `()` type by default
- Most things in Rust are expressions
- The last value in an expression is returned from that expression
- `dbg!` returns whatever we give it, as is.

That means we're left with a main function with `dbg!` in the last position, which returns whatever we gave it. In this case, that was the api token: a `String`.

This lets us read the error message we got from the Rust compiler. First, the compiler expected `()` because that's a function's default return type.

```
  |
1 | fn main() {
  |           - expected `()` because of default return type
```

Then, on line 4, the compiler expected the `()` value because the default return type of the function told it to. Instead, it found that we are returning a `String` because `dbg!` returns whatever we give it, and `dbg!` is in the last position in the function body, which means whatever value it is, is what gets returned from the function.

```
4 |     dbg!(api_token)
  |     ^^^^^^^^^^^^^^^ expected `()`, found struct `String`
  |
```

So the question becomes how do we discard the `dbg!` value and the answer is the semicolon: `;`.

A semicolon is an operator that takes an expression, evaluates it, then discards the result. In place of the discarded result, we get `()`, which is the return type we need for the `main` function.

This is why the semicolon is important. It takes our `dbg!` expression and evaluates it which lets us print to the console, while also letting us return `()`.