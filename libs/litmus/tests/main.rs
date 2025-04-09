use std::{collections::HashSet, marker::PhantomData, process::ExitCode};
use libtest::{Arguments, Failed};
use litmus::{elements::Scenario, InfallibleExt};

pub fn main() -> ExitCode {
    type World = RepositoryWorld<usize, InMemoryRepositoryWorldHandle<usize>>;
    type GivenFn = fn(&mut World) -> Result<(), Failed>;
    type WhenFn = fn(&mut World) -> Result<(), Failed>;
    type ThenFn = fn(&World) -> Result<(), Failed>;

    let args = Arguments::from_args();
    let tests = vec![
        Scenario::<World>::unnamed()
            .given::<GivenFn>(
                "an empty repository", 
                |_: &mut World| Ok(())
            )
            .when::<WhenFn>(
                "adding task 0", 
                (|world: &mut World| { world.repository.add(0); Ok(()) })
            )
            .then::<ThenFn>(
                "the repository should contain task 0", 
                |world: &World| {
                    if !world.repository.contains(&0) {
                        return Err("Expected task 0 to be present, found absent".into());
                    }
                    Ok(())
                }
            )
    ];

    litmus::run(&args, tests).exit_code()
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
