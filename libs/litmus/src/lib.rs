pub mod elements;
mod utils;

pub use self::utils::extensions::InfallibleExt;

pub fn run<WorldImpl>(args: &libtest::Arguments, tests: Vec<impl Into<libtest::Trial>>) -> libtest::Conclusion {
    let tests = tests
        .into_iter()
        .map(Into::into)
        .collect();

    libtest::run(args, tests)
}
