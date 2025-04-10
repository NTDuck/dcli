pub mod assertions;
pub mod elements;
pub mod prelude;
mod utils;

pub fn run(args: &libtest::Arguments, trials: impl Into<Vec<libtest::Trial>>) -> libtest::Conclusion {
    libtest::run(args, trials.into())
}

pub fn run_with_cli_args(trials: impl Into<Vec<libtest::Trial>>) -> libtest::Conclusion {
    let args = libtest::Arguments::from_args();
    run(&args, trials.into())
}
