use crate::elements::Tag;
use crate::utils::aliases::{Arc, HashMap};

use super::World;

#[derive(Default)]
pub(super) struct Hooks<WorldImpl> {
    pub(super) tagged: HashMap<Tag, Arc<dyn HookFn<WorldImpl>>>,
    pub(super) untagged: Vec<Arc<dyn HookFn<WorldImpl>>>,
    cached: Option<Arc<dyn HookFn<WorldImpl>>>,
}

impl<WorldImpl> Clone for Hooks<WorldImpl> {
    fn clone(&self) -> Self {
        Self {
            tagged: self.tagged.clone(),
            untagged: self.untagged.clone(),
            cached: self.cached.clone(),
        }
    }
}

impl<WorldImpl> Hooks<WorldImpl> 
where 
    WorldImpl: World,
{
    pub(super) fn cache(self, hook: impl HookFn<WorldImpl>) -> Self {
        let mut untagged = self.untagged;
        let mut cached = self.cached;

        if let Some(cached) = cached.take() {
            untagged.push(cached);
        }

        let hook = Arc::new(hook);

        Self {
            tagged: self.tagged,
            untagged,
            cached: Some(hook),
        }
    }

    pub(super) fn tagged<U>(self, tags: impl IntoIterator<Item = U>) -> Self
    where
        U: Into<Tag>,
    {
        if let Some(cached) = self.cached {
            let mut tagged = self.tagged;

            tags
                .into_iter()
                .map(Into::into)
                .map(|tag| tagged.insert(tag, cached.clone()));
    
            Self {
                tagged,
                untagged: self.untagged,
                cached: None,
            }

        } else {
            unreachable!()
        }
    }

    pub(super) fn untagged(self) -> Self {
        let mut untagged = self.untagged;
        let mut cached = self.cached;

        if let Some(cached) = cached.take() {
            untagged.push(cached);
        }

        Self {
            tagged: self.tagged,
            untagged,
            cached: None,
        }
    }
}

pub trait HookFn<WorldImpl>: Fn(&mut WorldImpl) -> () + Send + Sync + 'static {}

impl<T, WorldImpl> HookFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> () + Send + Sync + 'static,
{
}

pub trait GlobalHookFn: FnOnce() -> () + Send + Sync + 'static {}

impl<T> GlobalHookFn for T
where 
    T: FnOnce() -> () + Send + Sync + 'static,
{
}
