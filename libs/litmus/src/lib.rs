pub mod elements;
mod utils;

pub fn run(args: impl AsRef<libtest::Arguments>, tests: impl Into<Vec<libtest::Trial>>) -> libtest::Conclusion {
    libtest::run(args.as_ref(), tests.into())
}
