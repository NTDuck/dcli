#[test]
pub fn main() {
    // Use a local struct to provide the test contexts with an environment.
    // The environment will contain the subject that is to be tested
    // along with any additional data you might need during the test run:
    #[derive(Clone, Default, Debug)]
    struct Environment {
        // ...
    }

    // `rspec::run(…)` is a convenience wrapper that takes care of setting up
    // a runner, logger, configuration and running the test suite for you.
    // If you want more direct control, you can manually set up those things, too.
    rspec::run(&rspec::given("rspec, a BDD testing framework", Environment::default(), |ctx| {
        // `given`, or any of its equivalents, opens the root context
        // of your test suite. Within you can then either define test examples:
        ctx.then("can define top-level tests", |_| true);

        // or make use of sub-contexts to add some structure to your test suite:
        ctx.when("contexts give your tests structure and reduce redundancy", |ctx| {
            ctx.before(|_| {
                // Executed once, before any of the contexts/examples is entered.
            });

            ctx.after(|_| {
                // Executed once, after all of the contexts/examples have been exited.
            });

            ctx.when("rspec can handle results", |ctx| {
                ctx.then("passes if the return is_ok()", |_| Ok(()) as Result<(),()>);
                // ctx.then("failes if the return is_err()", |_| Err(()) as Result<(),()>);
            });

            ctx.when("rspec can handle bools", |ctx| {
                ctx.then("should pass if true", |_| true);
                // ctx.then("should fail if false", |_| false);
                ctx.then("is convenient for comparisons", |_| (42 % 37 + 2) > 3);
            });

            ctx.when("rspec can handle units", |ctx| {
                ctx.then("should pass if the return is ()", |_| {});
            });

            ctx.when("rspec can handle panics", |ctx| {
                ctx.then("is convenient for asserts", |_| assert_eq!(1, 1));
            });
        });
    })); // exits the process with a failure code if one of the tests failed.
}