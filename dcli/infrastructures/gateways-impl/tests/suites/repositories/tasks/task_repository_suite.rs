macro_rules! suite {
    (handle = $handle:ident) => {
        use crate::suites::repositories::tasks::task_repository_suite::given;
        use crate::suites::repositories::tasks::task_repository_suite::then;
        use crate::suites::repositories::tasks::task_repository_suite::when;
        use crate::worlds::repositories::tasks::TaskRepositoryWorld;

        type World = TaskRepositoryWorld<$handle>;

        #[test]
        fn f0() {
            given!(an empty repository);
            when!(adding task {0});
            then!(the repository contains task {0});
        }

        #[test]
        fn f1() {
            given!(an empty repository);
            when!(removing task {0});
            then!(the repository is empty);
        }

        #[test]
        fn f2() {
            given!(a repository containing tasks {0} to {1024});
            when!(removing task {444});
            then!(the repository contains task {0} to {1024} except {444});
        }
    };
}

macro_rules! given {
    (an empty repository) => {
        let mut world = World::given_empty_repository();
    };

    (a repository containing tasks { $idx_start:expr }to { $idx_end:expr }) => {
        let mut world =
            World::given_repository_with_task_range($idx_start..=$idx_end);
    };
}

macro_rules! when {
    (adding task { $idx:expr }) => {
        world.when_adding_task($idx);
    };

    (removing task { $idx:expr }) => {
        world.when_removing_task($idx);
    };
}

macro_rules! then {
    (the repository is empty) => {
        world.then_repository_is_empty();
    };

    (the repository contains task { $idx:expr }) => {
        world.then_repository_contains_only_task($idx);
    };

    (
        the repository contains task { $idx_start:expr }to { $idx_end:expr }except { $idx_except:expr }
    ) => {
        world.then_repository_contains_task_range_except(
            $idx_start..=$idx_end,
            $idx_except,
        );
    };
}

pub(crate) use given;
pub(crate) use suite;
pub(crate) use then;
pub(crate) use when;
