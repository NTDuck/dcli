use std::{collections::HashSet, marker::PhantomData, process::ExitCode};
use litmus::elements::{Background, Feature, Rule, Scenario};
use litmus::prelude::*;

pub fn main() -> ExitCode {
    type World = RepositoryWorld<usize, InMemoryRepositoryWorldHandle<usize>>;

    let trials = Feature::named("Number Repository")
        .ignored(false)
        .background(Background::named("Nothingness")
            .ignored(true)
            .given("nothing", |_| ok())
            .and("nothing", |_| ok())
            .but("still nothing", |_| ok()))
        .rule(|ctx| Rule::from(ctx)
            .named("Big numbers should work as well")
            // .ignored(true)
            .background(Background::<World>::named("Populate repository with big numbers")
                .given("a repository with task `MAX`", |w| w.repository.add(usize::MAX).ok())
                .and("a repository with task `MAX - 1`", |w| w.repository.add(usize::MAX - 1).ok())
                .and("a repository with task `MAX - 2`", |w| w.repository.add(usize::MAX - 2).ok()))
            .scenario(|ctx| Scenario::from(ctx)
                .unnamed()
                .given("a populated repository", |_| ok())
                .when("doing nothing", |_| ok())
                .then("the repository should contain all populated numbers", |w| {
                    w.repository.contains(&usize::MAX)
                        .expect(true, "expected `MAX` to be present, found absent")?;
                    w.repository.contains(&(usize::MAX - 1))
                        .expect(true, "expected `MAX - 1` to be present, found absent")?;
                    w.repository.contains(&(usize::MAX - 2))
                        .expect(true, "expected `MAX - 2` to be present, found absent")?;

                    Ok(())
                })))
        .scenario(|ctx| Scenario::<_, _, World>::from(ctx)
            .unnamed()
            .given("an empty repository", |_| ok())
            .when("adding 0", |w| w.repository.add(0).ok())
            .then("the repository should contain 0", |w| w.repository.contains(&0)
                .expect(true, "expected 0 to be present, found absent")))
        .scenario(|ctx| Scenario::<_, _, World>::from(ctx)
            .unnamed()
            .given("a repository containing 0", |w| w.repository.add(0).ok())
            .and("1", |w| w.repository.add(1).ok())
            .when("removing 0", |w| w.repository.remove(&0).ok())
            .but("not removing 1", |_| ok())
            .then("the repository should contain 1", |w| w.repository.contains(&1)
                .expect(true, "expected 1 to be present, found absent"))
            .but("not 0", |w| w.repository.contains(&0)
                .expect(false, "expected 0 to be absent, found present")))
        ;

    litmus::run(trials).exit_code()
}

pub trait Repository<T> {
    fn add(&mut self, item: T);
    fn remove(&mut self, item: &T);
    fn clear(&mut self);

    fn contains(&self, item: &T) -> bool;
}

pub struct RepositoryWorld<T, Handle: RepositoryWorldHandle<T>> {
    pub repository: Handle::Repository,
}

impl<T, Handle: RepositoryWorldHandle<T>> Default for RepositoryWorld<T, Handle> {
    fn default() -> Self {
        Self {
            repository: Handle::default(),
        }
    }
}

pub trait RepositoryWorldHandle<T> {
    type Repository: Repository<T>;

    fn default() -> Self::Repository;
}

pub struct InMemoryRepository<T>(HashSet<T>);

impl<T> Default for InMemoryRepository<T> {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

impl<T> Repository<T> for InMemoryRepository<T>
where
    T: Eq + std::hash::Hash,
{
    fn add(&mut self, item: T) {
        self.0.insert(item);
    }
    
    fn remove(&mut self, item: &T) {
        self.0.remove(item);
    }
    
    fn clear(&mut self) {
        self.0.clear();
    }
    
    fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }
}

pub struct InMemoryRepositoryWorldHandle<T>(PhantomData<T>);

impl<T> RepositoryWorldHandle<T> for InMemoryRepositoryWorldHandle<T>
where
    T: Eq + std::hash::Hash,
{
    type Repository = InMemoryRepository<T>;
    
    fn default() -> Self::Repository {
        Self::Repository::default()
    }
}
