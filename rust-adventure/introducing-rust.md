> In our email, we created a file called src/main.rs and a function in that file called fn main. What happens if either of these are called something else?

## Answers

### No main.rs file

The name and location of the `main.rs` file is important, but only because we haven't configured which file to use as the entrypoint for the binary target. If we don't specify the target details in `Cargo.toml` and we don't have the default `src/main.rs` file, then we see this error message from the compiler when we build.

```
❯ cargo build
error: failed to parse manifest at `/Users/chris/weather-cli/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```

This message is telling us that we need at least one library or binary target to be able to build the project. If we don't want to use `src/main.rs`, we can specify the binary by adding a `[[bin]]` field in our `Cargo.toml`

```toml
[[bin]]
name = "weather-cli"
path = "src/my-binary-file.rs"
```

This is also how you'd specify a number of other configuration options for how to build a specific target, such as the name of the binary that gets built.

### No main function

If we rename the main function in `main.rs` to something nonsensical like `fn asfasf()` then when we try to build we'll see this error message.

```
❯ cargo build
   Compiling thingd v0.1.0 (/Users/chris/weather-cli)
error[E0601]: `main` function not found in crate `weather-cli`
 --> src/main.rs:1:1
  |
1 | / fn asfasf() {
2 | |     println!("Hello, world!");
3 | | }
  | |_^ consider adding a `main` function to `src/main.rs`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
error: could not compile `thingd`

To learn more, run the command again with --verbose.
```

The most important piece of the error message is this line:

> "consider adding a `main` function to `src/main.rs`"

This error occurs because we must have a function called `main` in the binary entrypoint file. It is a convention enforced by the compiler.

In the future when we get into async Rust and other higher level macros, this error can pop up if you've misconfigured a macro that wraps the main function.