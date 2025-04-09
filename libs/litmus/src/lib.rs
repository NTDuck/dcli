pub mod elements;
mod utils;

use elements::World;

pub use self::utils::extensions::InfallibleExt;

pub fn run<WorldImpl>(args: &libtest::Arguments, tests: Vec<impl IntoTrial<WorldImpl>>) -> libtest::Conclusion
where
    WorldImpl: World,
{
    let tests = tests
        .into_iter()
        .map(IntoTrial::into_trail)
        .collect();

    libtest::run(args, tests)
}

pub trait IntoTrial<WorldImpl>
where
    WorldImpl: World,
{
    fn into_trail(self) -> libtest::Trial;
}
