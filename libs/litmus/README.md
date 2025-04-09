# litmus

a macro-free BDD test harness.<br>
inspired by [cucumber](https://crates.io/crates/cucumber) and [rspec](https://crates.io/crates/rspec).

## Major design criteria
- works with [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [cargo-nextest](https://nexte.st)
- offers [Gherkin](https://cucumber.io/docs/gherkin/) ergonomics
- is [fast](https://blog.codinghorror.com/performance-is-a-feature/)

## Examples
```
cargo test --example main -p litmus
cargo nextest run --example main -p litmus
```
