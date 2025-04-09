pub mod assertions;
pub mod elements;
pub mod prelude;
mod utils;

pub fn run(args: &libtest::Arguments, tests: Vec<impl Into<libtest::Trial>>) -> libtest::Conclusion {
    let tests = tests
        .into_iter()
        .map(Into::into)
        .collect();

    libtest::run(args, tests)
}
