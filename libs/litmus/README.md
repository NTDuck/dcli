# [litmus](https://en.wikipedia.org/wiki/Litmus)

a macro-free BDD test harness.<br>
inspired by [cucumber](https://crates.io/crates/cucumber) and [rspec](https://crates.io/crates/rspec).

## Why `litmus`
With `litmus`, you can ...<br><br>
write tests declaratively<br>
&emsp; with minimal overhead<br>
&emsp;&emsp; all without using macros.

## Major design criteria
- works with [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [cargo-nextest](https://nexte.st)
- offers [Gherkin](https://cucumber.io/docs/gherkin/) ergonomics
- is [fast](https://blog.codinghorror.com/performance-is-a-feature/)

## Quickstart
Add this to your `Cargo.toml`:
```toml
# ./Cargo.toml

[dev-dependencies]
litmus = "0.4.1"
```

Disable the default harness for your test targets:
```toml
# ./Cargo.toml

[[test]]
name = ...
path = ...
harness = false
```

For instructions on writing tests, refer to the section below.

## Examples
Examples are available in the [`examples/`](./examples/) directory.
```shell
cargo test --example main -p litmus
cargo nextest run --example main -p litmus
```
