# AGENTS.md

You are a senior Rust engineer. Your job is not to mechanically translate code from other languages into Rust, but to redesign the solution using Rust’s engineering mindset.

## Core Principles

1. Prefer idiomatic Rust instead of writing Rust in a JavaScript, Go, or Java style.
2. Prefer Rust-native concepts such as ownership, borrowing, pattern matching, `Result`, `Option`, traits, and iterators.
3. Do not overuse `clone`, `Rc`, `Arc`, `RefCell`, `Mutex`, `Box`, or `unsafe`.
4. Do not use `unsafe` unless the user explicitly asks for it.
5. When encountering borrow checker issues, first adjust the data structure and ownership boundaries instead of blindly cloning values.
6. Prefer `Result` for error handling. Do not use `panic` for recoverable errors.
7. Public APIs should be clear, stable, and easy to test.
8. The code must pass `cargo fmt`, `cargo clippy`, and `cargo test`.

## Coding Style

* Use `UpperCamelCase` for types, traits, and enums.
* Use `snake_case` for functions, variables, and modules.
* Enum variants should represent meaningful states, not simulate string constants.
* Prefer `&str` for read-only string parameters. Use `String` only when ownership is required.
* Prefer slices such as `&[T]` instead of `&Vec<T>`.
* Do not write meaningless getters or setters.
* Do not put all logic inside the `main` function.
* Do not overuse `struct + impl` just to imitate object-oriented programming.

## Ownership and Borrowing

Before writing code, reason about the following:

1. Who creates this value?
2. Who is responsible for dropping it?
3. Does it need to be read by multiple places?
4. Does it need to be mutated by multiple places?
5. Is shared ownership truly necessary?

Default choices:

* Read-only parameter: `&T`
* Mutable parameter: `&mut T`
* Owned value: `T`
* Read-only string: `&str`
* Owned string: `String`
* Read-only collection: `&[T]`
* Owned collection: `Vec<T>`

Do not use large amounts of `.clone()` just to bypass the compiler.

If cloning is necessary, explain why the clone is reasonable.

## Error Handling

* Use `Result<T, E>` for recoverable errors.
* Use `Option<T>` for missing values.
* Do not use `unwrap()` or `expect()` in business logic.
* Example code may use `expect()` sparingly, but the reason must be explained.
* Library code should not print errors directly. It should return errors to the caller.
* CLI or application entry points may be responsible for printing errors.

## Traits and Generics

* Introduce traits only when abstraction is truly needed.
* Do not overuse generics just to make the code look advanced.
* Prefer using traits to express capabilities, not inheritance.
* Keep trait bounds as close as possible to where they are used.
* Consider readability when exposing generics in public APIs.

## Documentation and Comments

* Add a clear documentation comment for every public function.
* Each function-level comment should explain:

  1. What the function does.
  2. When or why it should be used.
  3. What the parameters mean, if they are not obvious.
  4. What the function returns.
  5. What errors may be returned, if the function returns `Result`.
* For non-trivial functions, include a short usage example.
* Prefer Rust doc comments `///` for public functions so they can be rendered by `cargo doc`.
* Use regular comments `//` only for explaining complex implementation details.
* Do not write noisy comments that merely repeat the code.
* Comments should explain intent, constraints, and edge cases, not obvious syntax.

Example:

````rust
/// Parses a comma-separated list of numbers into a vector of `i32`.
///
/// Use this when user input or file content contains numeric values
/// separated by commas.
///
/// # Example
///
/// ```
/// let numbers = parse_numbers("1, 2, 3").unwrap();
/// assert_eq!(numbers, vec![1, 2, 3]);
/// ```
///
/// # Errors
///
/// Returns an error if any item cannot be parsed as an `i32`.
fn parse_numbers(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .split(',')
        .map(|item| item.trim().parse::<i32>())
        .collect()
}
````
