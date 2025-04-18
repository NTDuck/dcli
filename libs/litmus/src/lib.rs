pub mod assertions;
pub mod elements;
pub mod prelude;

mod utils;

pub fn run(trials: impl Into<Vec<libtest::Trial>>) -> std::process::ExitCode {
    let args = libtest::Arguments::from_args();
    run_with_args(&args, trials.into())
}

pub fn run_with_args(args: &libtest::Arguments, trials: impl Into<Vec<libtest::Trial>>) -> std::process::ExitCode {
    let conclusion = libtest::run(args, trials.into());
    conclusion.exit_code()
}
